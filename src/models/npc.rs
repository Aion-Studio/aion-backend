use std::{any::Any, cmp::max};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot, watch,
};
use tracing::{error, log::info};

use crate::events::combat::{CombatError, CombatantIndex, CombatantState};
use crate::models::cards::{Card, Deck};
use crate::{
    events::combat::CombatTurnMessage,
    prisma::npc,
    services::{
        impls::combat_service::CombatCommand, traits::combat_decision_maker::DecisionMaker,
    },
};

use super::{
    combatant::Combatant,
    hero::Range,
    resources::Relic,
    talent::{Spell, Talent},
};

#[derive(Debug)]
pub struct CpuCombatantDecisionMaker {
    command_sender: Option<Sender<CombatCommand>>,
    // result_receiver: Arc<Mutex<Receiver<CombatTurnMessage>>>,
    monster: Monster,
    player_idx: CombatantIndex,
    combat_controller_tx: Option<Sender<CombatCommand>>,
    shutdown_signal: Option<watch::Receiver<bool>>,
    shutdown_trigger: Option<watch::Sender<bool>>,
    encounter_id: String,
}

impl CpuCombatantDecisionMaker {
    pub(crate) fn new(monster: Monster, encounter_id: String) -> Self {
        let (shutdown_trigger, shutdown_signal) = watch::channel(false);
        Self {
            monster,
            player_idx: CombatantIndex::Npc,
            combat_controller_tx: None,
            shutdown_signal: Some(shutdown_signal),
            shutdown_trigger: Some(shutdown_trigger),
            encounter_id,
            command_sender: None,
        }
    }
    pub fn get_command_tx(&self) -> Option<Sender<CombatCommand>> {
        self.command_sender.clone()
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
        let (result_sender, mut result_receiver) = mpsc::channel(10);

        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let mut shutdown_rx1 = shutdown_rx.clone();

        let shutdown_signal = self
            .shutdown_signal
            .take()
            .expect("Shutdown signal must be present when starting.");

        let handle_1 = tokio::spawn(async move {
            let npc_player_idx = idx.clone();
            tokio::select! {
            _ = shutdown_rx1.changed() => {
                info!("Shutting down signal monster (first task).");
            },
            _ = async {
                let mut llm_decisions: Vec<CombatCommand> = vec![]; // Initialize a local decisions vector.

                while let Some(result) = result_receiver.recv().await {
                    match result {
                        CombatTurnMessage::PlayerTurn(turn_idx) => {
                            // Do nothing
                                info!("NPC player turn {:?}", turn_idx);
                        }
                        _=>{}
                    };
                }
            } => {}
            }
        });

        let (command_tx, mut command_receiver) = mpsc::channel(10);

        self.command_sender = Some(command_tx);
        let combat_sender = combat_controller_tx.clone();

        let id = self.encounter_id.clone();

        let mut shutdown_rx2 = shutdown_rx.clone();
        let handle_2 = tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(cmd) = command_receiver.recv() => {
                        if let Err(e) = combat_sender.send(cmd).await {
                            error!("error in sending command to combat controller");
                        }
                    },
                    _ = tokio::signal::ctrl_c() => {
                        // Handle Ctrl+C or another termination signal if needed
                       break;
                    },
                    _ = shutdown_rx2.changed() => {
                        info!("Received internal shutdown signal, terminating command handling task.");
                        break;
                    },
                }
            }
        });

        self.shutdown_trigger = Some(shutdown_tx);

        info!("returning result sender of monster");
        result_sender
    }
    fn get_id(&self) -> String {
        info!(
            "im gonna show you my props {:?} {:?} ",
            self.shutdown_signal, self.monster
        );

        self.monster.get_id()
    }
    fn shutdown(&mut self) {
        if let Some(trigger) = self.shutdown_trigger.take() {
            let _ = trigger.send(true);
        }
    }
}
impl Drop for CpuCombatantDecisionMaker {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Monster {
    id: String,
    name: String,
    pub damage: Range<i32>,
    pub hit_points: i32,
    pub max_hp: i32,
    pub armor: i32,
    pub level: i32,
    pub mana: i32,
    cards_in_discard: Vec<Card>,
    cards_in_hand: Vec<Card>,
    monster_type: Option<String>,
    pub talents: Vec<Talent>,
}

impl Default for Monster {
    fn default() -> Self {
        let hp = 50;
        Monster {
            id: "1".to_string(),
            name: "Goblin".to_string(),
            damage: Range { min: 1, max: 3 },
            // random between 20-40
            hit_points: hp,
            max_hp: hp,
            armor: 0,
            level: 1,
            mana: 0,
            cards_in_discard: vec![],
            cards_in_hand: vec![],
            monster_type: None,
            talents: vec![],
        }
    }
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

    fn get_mana(&self) -> i32 {
        self.mana
    }

    fn heal(&mut self, amount: i32) {
        self.hit_points = max(self.hit_points, self.hit_points + amount);
    }

    fn get_player_state(&self) -> CombatantState {
        CombatantState::Npc {
            hp: self.get_hp(),
            max_hp: self.max_hp,
            spells: self.get_spells(),
        }
    }

    fn get_cards_in_discard(&self) -> &Vec<Card> {
        &self.cards_in_discard
    }

    fn get_spells(&self) -> Vec<Spell> {
        vec![]
    }

    fn get_zeal(&self) -> i32 {
        0
    }

    fn get_armor(&self) -> i32 {
        self.armor
    }

    fn get_level(&self) -> i32 {
        self.level
    }

    fn take_damage(&mut self, damage: i32) {
        let damage = max(0, damage - self.armor);
        self.hit_points -= damage;
    }

    fn shuffle_deck(&mut self) {}

    fn draw_cards(&mut self) {
        // Ensure the deck exists and determine if we need to shuffle
    }

    fn add_to_discard(&mut self, _: Card) {}
    fn add_mana(&mut self) {
        self.mana = 3;
    }

    fn spend_mana(&mut self, mana: i32) {
        self.mana -= mana;
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.cards_in_hand
    }

    fn get_relics(&self) -> Vec<Relic> {
        vec![]
    }

    fn play_card(&mut self, card: &Card) -> Result<(), CombatError> {
        if let Some(idx) = self.cards_in_hand.iter().position(|c| c.id == card.id) {
            self.cards_in_hand.remove(idx);
            // self.add_to_discard(card.clone());
            Ok(())
        } else {
            Err(CombatError::CardNotInHand)
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
            level: data.level,
            hit_points: data.hp,
            max_hp: data.hp,
            armor: data.armor,
            monster_type: None,
            mana: 0,
            cards_in_hand: vec![],
            cards_in_discard: vec![],
            talents: vec![], // nothing for now
        }
    }
}

impl From<Box<npc::Data>> for Monster {
    fn from(data: Box<npc::Data>) -> Self {
        Monster::from(*data)
    }
}
