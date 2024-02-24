use std::any::Any;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot, Mutex,
};
use tracing::log::info;

use crate::events::combat::CombatantIndex;
use crate::{
    events::combat::CombatTurnMessage,
    prisma::npc,
    services::{
        impls::combat_service::CombatCommand, traits::combat_decision_maker::DecisionMaker,
    },
};

use super::{combatant::Combatant, hero::Range, talent::Talent};

#[derive(Debug)]
pub struct CpuCombatantDecisionMaker {
    // command_sender: Sender<CombatCommand>,
    // result_receiver: Arc<Mutex<Receiver<CombatTurnMessage>>>,
    monster: Monster,
    player_idx: CombatantIndex,
    combat_controller_tx: Option<Sender<CombatCommand>>,
    shutdown_signal: Option<oneshot::Receiver<()>>,
    shutdown_trigger: Option<oneshot::Sender<()>>,
}

impl CpuCombatantDecisionMaker {
    pub(crate) fn new(
        // command_sender: Sender<CombatCommand>,
        // result_receiver: Receiver<CombatTurnMessage>,
        monster: Monster,
    ) -> Self {
        let (shutdown_trigger, shutdown_signal) = oneshot::channel();

        Self {
            // command_sender,
            // result_receiver: Arc::new(Mutex::new(result_receiver)),
            monster,
            player_idx: CombatantIndex::Combatant2,
            combat_controller_tx: None,
            shutdown_signal: Some(shutdown_signal),
            shutdown_trigger: Some(shutdown_trigger),
        }
    }
}

impl DecisionMaker for CpuCombatantDecisionMaker {
    fn start(
        &mut self,
        combat_controller_tx: Sender<CombatCommand>,
        idx: CombatantIndex,
    ) -> Sender<CombatTurnMessage> {
        self.player_idx = idx.clone();
        self.combat_controller_tx = Some(combat_controller_tx.clone());
        let (command_sender, result_receiver) = mpsc::channel(10);
        let combat_sender = combat_controller_tx.clone();
        let shutdown_signal = self
            .shutdown_signal
            .take()
            .expect("Shutdown signal must be present when starting.");

        tokio::spawn(async move {
            let mut result_receiver = result_receiver;
            let npc_player_idx = idx.clone();
            tokio::select! {
                _ = shutdown_signal => {
                    info!("Shutting down decision maker for monster.");
                },
                _ = async {
                    while let Some(result) = result_receiver.recv().await {
                        match result {
                            CombatTurnMessage::PlayerTurn(turn_idx) => {
                                // Do nothing
                                if npc_player_idx == turn_idx {
                                    let command = CombatCommand::Attack; // Example decision
                                    combat_sender
                                        .clone()
                                        .send(command)
                                        .await
                                        .expect("Failed to send command");
                                }
                            }
                            CombatTurnMessage::Winner(idx) => {
                                // Do nothing
                            }
                            CombatTurnMessage::CommandPlayed(opponent_state) => {
                                // Do nothing for now
                            }
                            _ => {}
                        };
                        // --------------------CPU logic to decide next move based on the received result
                            // Existing logic to handle combat results
                    }
                } => {}
            }
        });
        info!("returning result sender of monster");
        command_sender
    }
    fn get_id(&self) -> String {
        info!("im gonna show you my props {:?} {:?} ",self.shutdown_signal, self.monster);
        
        self.monster.get_id()
    }

    fn shutdown(&mut self) {
        if let Some(trigger) = self.shutdown_trigger.take() {
            let _ = trigger.send(());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Monster {
    id: String,
    name: String,
    pub damage: Range<i32>,
    pub hit_points: i32,
    pub armor: i32,
    pub level: i32,
    monster_type: Option<String>,
    pub talents: Vec<Talent>,
}

impl Combatant for Monster {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_hp(&self) -> i32 {
        self.hit_points
    }
    fn get_damage(&self) -> i32 {
        self.damage.roll()
    }

    fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }

    fn get_armor(&self) -> i32 {
        self.armor
    }

    fn get_level(&self) -> i32 {
        self.level
    }

    fn attack(&self, other: &mut dyn Combatant) {
        let damage = self.damage.roll();
        other.take_damage(damage);
    }

    fn take_damage(&mut self, damage: i32) {
        self.hit_points -= damage;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<npc::Data> for Monster {
    fn from(data: npc::Data) -> Self {
        Monster {
            id: data.id,
            name: data.name,
            damage: Range {
                min: data.damage_min,
                max: data.damage_max,
            },
            level: data.level,
            hit_points: data.hp,
            armor: data.armor,
            monster_type: None,
            talents: vec![], // nothing for now
        }
    }
}
