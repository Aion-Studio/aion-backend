use std::{any::Any, cmp::max};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};
use tracing::log::info;

use crate::events::combat::{CombatError, CombatantIndex};
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
    talent::{Spell, Talent},
};

#[derive(Debug)]
pub struct CpuCombatantDecisionMaker {
    // command_sender: Sender<CombatCommand>,
    // result_receiver: Arc<Mutex<Receiver<CombatTurnMessage>>>,
    monster: Monster,
    player_idx: CombatantIndex,
    combat_controller_tx: Option<Sender<CombatCommand>>,
    shutdown_signal: Option<oneshot::Receiver<()>>,
    shutdown_trigger: Option<oneshot::Sender<()>>,
    encounter_id: String,
}

impl CpuCombatantDecisionMaker {
    pub(crate) fn new(monster: Monster, encounter_id: String) -> Self {
        let (shutdown_trigger, shutdown_signal) = oneshot::channel();

        Self {
            monster,
            player_idx: CombatantIndex::Combatant2,
            combat_controller_tx: None,
            shutdown_signal: Some(shutdown_signal),
            shutdown_trigger: Some(shutdown_trigger),
            encounter_id,
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

        let id = self.encounter_id.clone();

        tokio::spawn(async move {
            let mut result_receiver = result_receiver;
            let npc_player_idx = idx.clone();
            tokio::select! {
                _ = shutdown_signal => {
                    info!("Shutting down signal monster.");
                },
                _ = async {
                    let mut llm_decisions: Vec<CombatCommand> = vec![]; // Initialize a local decisions vector.

                    while let Some(result) = result_receiver.recv().await {
                        match result {
                            CombatTurnMessage::PlayerTurn(turn_idx) => {
                                // Do nothing
                            }
                            CombatTurnMessage::Winner(idx) => {
                                // Do nothing
                            }
                            CombatTurnMessage::PlayerState{
                                me,
                                me_idx,
                                opponent,
                                opponent_battle_field,
                                my_battle_field,
                                active_effects,
                                turn,
                                opponent_hp
                            }=>{
                                if turn == idx.clone() {
                                     let cards_i_can_play = me.get_hand().iter().filter(|c| c.cost <= me.get_mana()).collect::<Vec<_>>();
                                    if llm_decisions.is_empty() {
                                        let client = reqwest::Client::new();

                                        let payload = json!({

                                        });

                                        let res = client.post("http://127.0.0.1:5000/message").json(&payload).send().await;
                                        let body = match res {
                                            Ok(res) => {
                                                match res.status() {
                                                    reqwest::StatusCode::OK => {
                                                        res.text().await.unwrap()
                                                    }
                                                    _ => {
                                                        println!("Error sending request: {:?}", res.status());
                                                        "".to_string()
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("Error sending request {:?}", e);
                                                "".to_string()
                                            }
                                        };
                                        let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();

                                        let actions_list = json_body["actions"].as_array().unwrap();
                                        if actions_list.is_empty() {
                                            llm_decisions.push(CombatCommand::EndTurn);
                                        }
                                    }
                                    // take first decision from the local decisions vector and send it to the combat controller
                                    let command = llm_decisions.remove(0);
                                    info!("NPC sending command {:?}", command);
                                    combat_sender
                                        .clone()
                                        .send(command)
                                        .await
                                        .expect("Failed to send command");
                                }
                            }
                            x => {
                            }
                        };
                    }
                } => {}
            }
        });
        info!("returning result sender of monster");
        command_sender
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
            let _ = trigger.send(());
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
    pub armor: i32,
    pub level: i32,
    pub mana: i32,
    cards_in_discard: Vec<Card>,
    cards_in_hand: Vec<Card>,
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
    fn get_mana(&self) -> i32 {
        self.mana
    }

    fn get_spells(&self) -> Vec<Spell> {
        vec![]
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
    fn get_hand(&self) -> &Vec<Card> {}
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
