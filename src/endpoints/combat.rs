use actix::prelude::*;
use actix_web::web::{Data, Query};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use prisma_client_rust::query_core::schema_builder::append_opt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot, Mutex, Notify};
use tracing::{error, info};

use crate::events::combat::{CombatEncounter, CombatError, CombatantIndex};

use crate::jsontoken::decode_token;
use crate::services::impls::combat_service::ControllerMessage;
use crate::{
    events::combat::CombatTurnMessage,
    services::{
        impls::combat_service::CombatCommand, traits::combat_decision_maker::DecisionMaker,
    },
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
    let claims = decode_token(&query.token);

    let combatant_id = match claims {
        Ok(claims) => claims.combatant_id,
        Err(e) => {
            error!("Failed to decode token: {}", e);
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };
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
            if let Some(state) = state {
                ws::start(
                    CombatSocket::new(combatant_id, Arc::new(Mutex::new(app_state)), state),
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
    // From Client --> Server
    Update(CombatTurnMessage), // From Server --> Client
}

#[derive(Clone)]
pub struct CombatSocket {
    combatant_id: String,
    app_state: Arc<Mutex<AppState>>,
    // Shared state to access CombatController potentially
    encounter_state: Option<CombatEncounter>,
    ws_to_player_decision_maker_tx: Option<Sender<CombatCommand>>,
}

impl CombatSocket {
    pub fn new(
        combatant_id: String,
        app_state: Arc<Mutex<AppState>>,
        turn_result: CombatTurnMessage,
    ) -> Self {
        // if turn result is a CombatTurnMessage::EncounterState
        // set the encounter state to Some(encounter_state)
        let encounter_state = match turn_result {
            CombatTurnMessage::EncounterState(state) => Some(state),
            _ => None,
        };

        CombatSocket {
            combatant_id,
            app_state,
            encounter_state,
            ws_to_player_decision_maker_tx: None,
        }
    }
}

impl Actor for CombatSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let (ws_sender, mut ws_receiver) = mpsc::channel(10);

        let decision_maker = PlayerDecisionMaker::new(self.combatant_id.clone(), ws_sender);
        let notify_handle = decision_maker.notify_from_ws_tx_set.clone();

        let decision_maker_arc = Arc::new(Mutex::new(decision_maker));

        let app_state = self.app_state.clone();
        let id = self.combatant_id.clone();

        let addr = ctx.address();
        // let decision_maker_clone = decision_maker.clone();

        let decision_maker_clone = decision_maker_arc.clone();
        ctx.spawn(fut::wrap_future(async move {
            let decision_maker = decision_maker_clone.clone();
            let state = app_state.lock().await;
            let (sender, _) = oneshot::channel();
            let tx = state.combat_tx.clone();
            // let mut decision_maker = decision_maker_clone.lock().await;

            let message = ControllerMessage::Combat((
                CombatCommand::EnterBattle(Some(decision_maker)),
                id.clone(),
                sender,
            ));
            let _ = tx.send(message).await;
            //make sure controller sets its sender
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

        // 2. (Optional) Retrieve initial combat state if the encounter's already begun
        if let Some(state) = self.encounter_state.clone() {
            ctx.text(
                serde_json::to_string(&CombatSocketMessage::Update(
                    CombatTurnMessage::EncounterState(state),
                ))
                .unwrap(),
            );
        }
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Logic to dissociate socket from combatant, clean up potentially?
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
        info!(
            "received combat turn message, serializeing and sending to client {:?}",
            msg
        );
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
                        _ => error!("Unexpected message type from client"),
                    },
                    Err(e) => {
                        error!("Received invalid message from client: {}", e);
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

        // 2. Assuming results come back to you eventually on an appropriate channel you receive a CombatTurnMessage
        // self.send_update(CombatTurnMessage, ctx); // Adjust how you receive results
    }
}

#[derive(Debug)]
pub struct PlayerDecisionMaker {
    id: String,
    combat_controller_tx: Option<Sender<CombatCommand>>,
    // provided by combat controller
    pub player_idx: CombatantIndex,
    to_ws_tx: Sender<CombatTurnMessage>,
    from_ws_tx: Option<Sender<CombatCommand>>,
    notify_from_ws_tx_set: Arc<Notify>,
    shutdown_signal: Option<oneshot::Receiver<()>>,
    shutdown_trigger: Option<oneshot::Sender<()>>,
}

impl PlayerDecisionMaker {
    fn new(id: String, to_ws_tx: Sender<CombatTurnMessage>) -> Self {
        let (shutdown_trigger, shutdown_signal) = oneshot::channel();

        Self {
            combat_controller_tx: None,
            id,
            to_ws_tx,
            player_idx: CombatantIndex::Combatant1,
            from_ws_tx: None,
            notify_from_ws_tx_set: Arc::new(Notify::new()),
            shutdown_signal: Some(shutdown_signal),
            shutdown_trigger: Some(shutdown_trigger),
        }
    }

    // Receive command from player websocket, send to combat controller
    pub fn start_listening_for_commands(&mut self) {
        let (command_sender, mut command_receiver) = mpsc::channel(10);
        //set the sender so combat socket can get it via get_from_ws_tx it to send commands from player
        self.from_ws_tx = Some(command_sender);
        let combat_controller_tx = self.combat_controller_tx.clone().unwrap();
        tokio::spawn(async move {
            while let Some(cmd) = command_receiver.recv().await {
                info!("Received command from ws_player: {:?}", cmd);
                combat_controller_tx.send(cmd).await.unwrap();
            }
        });
    }
    pub fn get_from_ws_tx(&self) -> Option<Sender<CombatCommand>> {
        self.from_ws_tx.clone()
    }
}

impl DecisionMaker for PlayerDecisionMaker {
    fn start(
        &mut self,
        combat_controller_tx: Sender<CombatCommand>,
        player_idx: CombatantIndex,
    ) -> Sender<CombatTurnMessage> {
        self.player_idx = player_idx;
        self.combat_controller_tx = Some(combat_controller_tx);

        let shutdown_signal = self
            .shutdown_signal
            .take()
            .expect("Shutdown signal must be present when starting.");

        self.start_listening_for_commands();
        self.notify_from_ws_tx_set.notify_one();

        let (command_sender, result_receiver) = mpsc::channel(10);

        let to_ws_tx = self.to_ws_tx.clone();

        tokio::spawn(async move {
            let mut result_receiver = result_receiver;
            tokio::select! {
                _ = shutdown_signal => {
                    info!("Shutting down decision maker for player.");
                },
                _ = async {
                    while let Some(result) = result_receiver.recv().await {
                        let to_ws_tx = to_ws_tx.clone();
                        to_ws_tx.send(result).await.unwrap();
                    }
                } => {}
            }
        });
        command_sender
    }
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn shutdown(&mut self) {
        if let Some(trigger) = self.shutdown_trigger.take() {
            let _ = trigger.send(());
        }
    }
}
