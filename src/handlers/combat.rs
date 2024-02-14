use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::error;

use crate::{
    events::combat::CombatTurnResult,
    services::{
        impls::combat_service::CombatCommand, traits::combat_decision_maker::DecisionMaker,
    },
    // ... Other imports relevant to your combat system
    webserver::AppState,
};

// Data exchanged over the WebSocket
#[derive(Serialize, Deserialize, Debug)]
enum CombatSocketMessage {
    Command(CombatCommand),   // From Client --> Server
    Update(CombatTurnResult), // From Server --> Client
}

#[derive(Clone)]
pub struct CombatSocket {
    combatant_id: String,
    app_state: Arc<Mutex<AppState>>, // Shared state to access CombatController potentially
}

impl CombatSocket {
    pub fn new(combatant_id: String, app_state: Arc<Mutex<AppState>>) -> Self {
        CombatSocket {
            combatant_id,
            app_state,
        }
    }

    // ... Helper methods for sending updates might be here depending on complexity
}

impl Actor for CombatSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // 1. Associate this socket with the combatant within the shared state
        // (You'll need your custom logic here)

        // 2. (Optional) Retrieve initial combat state if the encounter's already begun
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Logic to dissociate socket from combatant, clean up potentially?
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for CombatSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if let Ok(message) = serde_json::from_str::<CombatSocketMessage>(&text) {
                    match message {
                        CombatSocketMessage::Command(command) => self.handle_command(command, ctx),
                        _ => error!("Unexpected message type from client"),
                    }
                } else {
                    // error handling, invalid JSON from the client
                }
            }
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => (), // Handle other message types (Binary), or potential close behavior
        }
    }
}

impl CombatSocket {
    fn handle_command(
        &mut self,
        command: CombatCommand,
        ctx: &mut <CombatSocket as actix::Actor>::Context,
    ) {
        // 1. Use app_state to access and forward your  command to the CombatController. Since commands could potentially come from both WebSockets and internally, we funnel them through here

        // 2. Assuming results come back to you eventually on an appropriate channel you receive a CombatTurnResult
        // self.send_update(CombatTurnResult, ctx); // Adjust how you receive results
    }

    fn send_update(
        &self,
        update: CombatTurnResult,
        ctx: &mut <CombatSocket as actix::Actor>::Context,
    ) {
        let serialized = serde_json::to_string(&CombatSocketMessage::Update(update)).unwrap();
        ctx.text(serialized);
    }
}

struct PlayerDecisionMaker {
    ws_sender: mpsc::Sender<CombatCommand>, // Sender part of a channel to communicate with the WebSocket session
    ws_receiver: Arc<Mutex<mpsc::Receiver<CombatTurnResult>>>, // Receiver part for listening to player commands
}

impl PlayerDecisionMaker {
    fn new(
        command_sender: mpsc::Sender<CombatCommand>,
        result_receiver: mpsc::Receiver<CombatTurnResult>,
    ) -> Self {
        Self {
            ws_sender: command_sender,
            ws_receiver: Arc::new(Mutex::new(result_receiver)),
        }
    }
}

impl DecisionMaker for PlayerDecisionMaker {
    fn listen_and_make_move(&mut self) {
        // Wait for the next command from the player via WebSocket
        let result_receiver = self.ws_receiver.clone();
        let sender = self.ws_sender.clone();
        tokio::spawn(async move {
            while let Ok(result) = result_receiver.lock().await.try_recv() {
                // ... your code to decide next move based on the received result ...
                let command = CombatCommand::Attack("target_id".to_string()); // Example decision
                sender.send(command).await.expect("Failed to send command");
            }
        });
    }
}
