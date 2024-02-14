use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

use crate::{
    events::combat::CombatTurnResult,
    prisma::npc,
    services::{
        impls::combat_service::CombatCommand, traits::combat_decision_maker::DecisionMaker,
    },
};

use super::{combatant::Combatant, hero::Range, talent::Talent};

struct CpuCombatantDecisionMaker {
    command_sender: Sender<CombatCommand>,

    result_receiver: Arc<Mutex<Receiver<CombatTurnResult>>>,
    monster: Monster,
}

impl CpuCombatantDecisionMaker {
    fn new(
        command_sender: Sender<CombatCommand>,
        result_receiver: Receiver<CombatTurnResult>,
        monster: Monster,
    ) -> Self {
        Self {
            command_sender,
            result_receiver: Arc::new(Mutex::new(result_receiver)),
            monster,
        }
    }
}

impl DecisionMaker for CpuCombatantDecisionMaker {
    fn listen_and_make_move(&mut self) {
        let command_sender = self.command_sender.clone();

        let result_receiver = self.result_receiver.clone();

        tokio::spawn(async move {
            while let Ok(result) = result_receiver.lock().await.try_recv() {
                // --------------------CPU logic to decide next move based on the received result
                let command = CombatCommand::Attack("target_id".to_string()); // Example decision
                command_sender
                    .send(command)
                    .await
                    .expect("Failed to send command");
            }
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
