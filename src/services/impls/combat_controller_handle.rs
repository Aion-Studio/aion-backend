use tokio::sync::{mpsc, oneshot};

use super::combat_service::ControllerMessage;

pub type CustomCommandResult = Result<(), Box<dyn std::error::Error + Send>>;

pub struct CombatControllerHandle {
    pub sender: mpsc::Sender<ControllerMessage>,
}

impl CombatControllerHandle {
    pub async fn start_encounter_for_combatant(&self, combatant_id: String) -> CustomCommandResult {
        match self
            .sender
            .send(ControllerMessage::StartEncounterForCombatant { combatant_id })
            .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("combat send err: {:?}", e);
                return Err(Box::new(e) as Box<dyn std::error::Error + Send>);
            }
        };
        Ok(())
    }
}
