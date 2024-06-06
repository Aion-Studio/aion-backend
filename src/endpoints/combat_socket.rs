use std::sync::Arc;

use actix::prelude::*;
use actix_web::web::{Data, Query};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::{error, info};

use crate::jsontoken::decode_combat_token;
use crate::models::player_decision_maker::PlayerDecisionMaker;
use crate::services::impls::combat_service::{ControllerMessage, EnterBattleData};
use crate::{
    events::combat::CombatTurnMessage,
    services::impls::combat_service::CombatCommand,
    // ... Other imports relevant to your combat system
    webserver::AppState,
};

#[derive(Deserialize)]
pub struct WsQueryParams {
    token: String,
}

#[get("/combat")]
pub async fn combat_ws(
    req: HttpRequest,
    stream: web::Payload,
    query: Query<WsQueryParams>,
    app_state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let claims = decode_combat_token(&query.token);

    let combatant_id = match claims {
        Ok(claims) => claims.combatant_id,
        Err(e) => {
            error!("Failed to decode token: {}", e);
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };
    info!("check combatant_id: {:?}", combatant_id);
    let app_state = app_state.get_ref().clone();
    let (tx, rx) = oneshot::channel();

    let msg = ControllerMessage::RequestState {
        combatant_id: combatant_id.clone(),
        tx,
    };

    app_state
        .combat_tx
        .send(msg)
        .await
        .map_err(|_| {
            error!("Failed to send combat state request");
            HttpResponse::InternalServerError().finish()
        })
        .expect("Failed to send combat state request");

    // Await the response from the combat controller
    match rx.await {
        Ok(state) => {
            if let Some(turn) = state.0 {
                ws::start(
                    CombatSocket::new(
                        combatant_id,
                        Arc::new(Mutex::new(app_state)),
                        (turn, state.1),
                    ),
                    &req,
                    stream,
                )
                .map_err(|_| actix_web::error::ErrorInternalServerError("WebSocket start failed"))
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    }
}

// Data exchanged over the WebSocket
#[derive(Serialize, Deserialize, Debug)]
enum CombatSocketMessage {
    Command(CombatCommand),
    Update(CombatTurnMessage), // From Server --> Client
}

#[derive(Clone)]
pub struct CombatSocket {
    combatant_id: String,
    app_state: Arc<Mutex<AppState>>,
    // Shared state to access CombatController potentially
    encounter_state: (CombatTurnMessage, Option<String>),
    ws_to_player_decision_maker_tx: Option<Sender<CombatCommand>>,
}

impl CombatSocket {
    pub fn new(
        combatant_id: String,
        app_state: Arc<Mutex<AppState>>,
        state: (CombatTurnMessage, Option<String>),
    ) -> Self {
        CombatSocket {
            combatant_id,
            app_state,
            encounter_state: state,
            ws_to_player_decision_maker_tx: None,
        }
    }
}

impl Actor for CombatSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let (ws_sender, mut ws_receiver) = mpsc::channel(10);

        let decision_maker = PlayerDecisionMaker::new(
            self.combatant_id.clone(),
            ws_sender,
            self.encounter_state.1.clone(),
        );
        let notify_handle = decision_maker.notify_from_ws_tx_set.clone();

        let decision_maker_arc = Arc::new(Mutex::new(decision_maker));

        let app_state = self.app_state.clone();
        let id = self.combatant_id.clone();

        let addr = ctx.address();

        let decision_maker_clone = decision_maker_arc.clone();
        ctx.spawn(fut::wrap_future(async move {
            let decision_maker = decision_maker_clone.clone();
            let state = app_state.lock().await;
            let (sender, _) = oneshot::channel();
            let tx = state.combat_tx.clone();

            let message = ControllerMessage::Combat((
                CombatCommand::EnterBattle(EnterBattleData(Some(decision_maker))),
                id.clone(),
                sender,
            ));
            let _ = tx.send(message).await;
            notify_handle.notified().await;

            let decision_maker_clone = decision_maker_clone.lock().await;
            let from_ws_tx = decision_maker_clone.get_from_ws_tx().unwrap();
            addr.clone()
                .do_send(SetWsToPlayerDecisionMakerTx(from_ws_tx));
        }));

        let addr = ctx.address();

        //  send messages to client from combat controller
        tokio::spawn(async move {
            while let Some(message) = ws_receiver.recv().await {
                addr.do_send(message); // sends to  impl Handler<CombatTurnMessage> handle fn
            }
        });

        ctx.text(
            serde_json::to_string(&CombatSocketMessage::Update(self.encounter_state.0.clone()))
                .unwrap(),
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let app_state = self.app_state.clone();
        let combatant_id = self.combatant_id.clone();

        tokio::spawn(async move {
            let combat_tx = app_state.lock().await.combat_tx.clone();
            combat_tx
                .send(ControllerMessage::RemoveSingleDecisionMaker { combatant_id })
                .await
                .unwrap();
        });
    }
}

struct SetWsToPlayerDecisionMakerTx(pub Sender<CombatCommand>);

impl Handler<SetWsToPlayerDecisionMakerTx> for CombatSocket {
    type Result = ();

    fn handle(&mut self, msg: SetWsToPlayerDecisionMakerTx, _: &mut Self::Context) -> Self::Result {
        self.ws_to_player_decision_maker_tx = Some(msg.0);
    }
}

impl Message for SetWsToPlayerDecisionMakerTx {
    type Result = ();
}

impl Handler<CombatTurnMessage> for CombatSocket {
    type Result = ();
    fn handle(&mut self, msg: CombatTurnMessage, ctx: &mut Self::Context) -> Self::Result {
        let serialized = serde_json::to_string(&CombatSocketMessage::Update(msg)).unwrap();
        ctx.text(serialized);
    }
}

// handle incoming client messsages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CombatSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<CombatSocketMessage>(&text) {
                    Ok(message) => match message {
                        CombatSocketMessage::Command(command) => self.handle_command(command),
                        _ => error!("Unexpected message type from client {:?}", message),
                    },
                    Err(e) => {
                        error!(
                            "Received invalid message from client: {} and the msg: {:?}",
                            e, text
                        );
                    }
                };
            }
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => (), // Handle other message types (Binary), or potential close behavior
        }
    }
}

impl CombatSocket {
    fn handle_command(&mut self, command: CombatCommand) {
        let _tx = self.ws_to_player_decision_maker_tx.clone();
        tokio::spawn(async move {
            if let Some(tx) = _tx {
                tx.send(command).await.unwrap();
            }
        });
    }
}
