use std::any::Any;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
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
}

impl CpuCombatantDecisionMaker {
    pub(crate) fn new(
        // command_sender: Sender<CombatCommand>,
        // result_receiver: Receiver<CombatTurnMessage>,
        monster: Monster,
    ) -> Self {
        Self {
            // command_sender,
            // result_receiver: Arc::new(Mutex::new(result_receiver)),
            monster,
            player_idx: CombatantIndex::Combatant2,
            combat_controller_tx: None,
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
        let (command_sender, mut result_receiver) = mpsc::channel(10);
        let combat_sender = combat_controller_tx.clone();

        tokio::spawn(async move {
            let npc_player_idx = idx.clone();
            while let Some(result) = result_receiver.recv().await {
                info!(
                    "monster received message from combat controller: {:?}",
                    result
                );
                match result {
                    CombatTurnMessage::PlayerTurn(turn_idx) => {
                        info!("monster received player turn message: {:?}", turn_idx);
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
            }
        });
        info!("returning result sender of monster");
        command_sender
    }
    fn get_id(&self) -> String {
        self.monster.get_id()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Monster {
    id: String,
    name: String,
    pub damage: Range<i32>,
    pub hit_points: i32,
    pub armor: i32,
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
        self.damage.min
    }

    fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }

    fn attack(&mut self, other: &mut dyn Combatant) {
        let damage = self.damage.roll();
        other.take_damage(damage);
    }

    fn take_damage(&mut self, damage: i32) {
        let damage = damage - self.armor;
        if damage > 0 {
            self.hit_points -= damage;
        }
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
            hit_points: data.hp,
            armor: data.armor,
            monster_type: None,
            talents: vec![], // nothing for now
        }
    }
}
