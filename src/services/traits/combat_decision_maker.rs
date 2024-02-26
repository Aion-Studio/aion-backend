use crate::events::combat::{CombatTurnMessage, CombatantIndex};
use crate::services::impls::combat_service::CombatCommand;
use tokio::sync::mpsc::{ Sender};


pub trait DecisionMaker: Send + Sync + std::fmt::Debug {
    fn start(
        &mut self,
        command_sender: Sender<CombatCommand>,
        player_idx: CombatantIndex,
    ) -> Sender<CombatTurnMessage>;

    fn get_id(&self) -> String;
    fn shutdown(&mut self);
}
