use std::{any::Any, cmp::max};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    mpsc::{self, Sender},
    oneshot,
};
use tracing::{log::info, warn};

use crate::events::combat::{CombatError, CombatantIndex};
use crate::models::cards::{Card, Deck};
use crate::prisma::DamageType;
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
                                     let cards_i_can_play = me.get_hand().iter().filter(|c| c.mana_cost <= me.get_mana()).collect::<Vec<_>>();
                                    if llm_decisions.is_empty() {
                                        let client = reqwest::Client::new();

                                        let my_hero_json = json!({
                                                "hp": me.get_hp(),
                                                "mana": me.get_mana(),
                                                "armor": me.get_armor(),
                                                "resilience": me.get_resilience(),
                                            });

                                        let opponent_json = json!({
                                                "hp": opponent_hp,
                                                "armor": opponent.get_armor(),
                                                "resilience": opponent.get_resilience(),
                                            });

                                        let payload = json!({
                                                "id": id,
                                                "payload": {
                                                    "my_battle_field": my_battle_field,
                                                    "opponent_battle_field": opponent_battle_field,
                                                    "cards_in_hand": cards_i_can_play,
                                                    "opponent_hero": opponent_json,
                                                    "my_hero": my_hero_json,
                                                }
                                        });


                                        info!("sending llm Cards in hand {:?}", cards_i_can_play);
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
                                        for action in actions_list {
                                            let action_type = action["type"].as_str().unwrap();
                                            let action_data = action["data"].as_object();

                                            match action_type {
                                                "EndTurn" => {
                                                    llm_decisions.push(CombatCommand::EndTurn);
                                                }
                                                "PlayCard" => {
                                                    let data = action_data.unwrap();
                                                    let card_id = data["card_id"].as_str().unwrap();
                                                    let card = cards_i_can_play.iter().find(|c| c.id == card_id).unwrap();
                                                    llm_decisions.push(CombatCommand::PlayCard((*card).clone()));
                                                }
                                                "Attack" => {
                                                    let data = action_data.unwrap();
                                                    let target_type = data["target_type"].as_str().unwrap();
                                                    let card_id = data["card_id"].as_str().unwrap();
                                                    let card = match my_battle_field.iter().find(|c| c.id == card_id) {
                                                        Some(card) => card,
                                                        None => {
                                                           cards_i_can_play.iter().find(|c| c.id == card_id).unwrap()
                                                        }
                                                    };

                                                    let card_owned: Card = (*card).clone();

                                                    if data["target_type"] == "Hero" {
                                                        llm_decisions.push(CombatCommand::AttackHero(card_owned));
                                                    } else {
                                                        let target_id = data["target_id"].as_str().unwrap();
                                                        llm_decisions.push(CombatCommand::AttackMinion{attacker: card_owned, defender_id: target_id.to_string()});
                                                    }
                                                }
                                                _ => {
                                                    warn!("Unknown action type: {:?}", action_type);
                                                    llm_decisions.push(CombatCommand::EndTurn);
                                                }
                                            }
                                        }
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
                                // info!("npc got some other message {:?}", x);
                            }
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
    pub resilience: i32,
    pub level: i32,
    pub mana: i32,
    pub deck: Option<Deck>,
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

    fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }

    fn get_damage_stats(&self) -> Range<i32> {
        self.damage.clone()
    }

    fn get_armor(&self) -> i32 {
        self.armor
    }

    fn get_resilience(&self) -> i32 {
        self.resilience
    }

    fn get_level(&self) -> i32 {
        self.level
    }

    fn take_damage(&mut self, damage: i32, damage_type: DamageType) {
        match damage_type {
            DamageType::Physical => {
                let diff = damage - self.armor;
                self.armor = max(0, self.armor - damage);
                if diff > 0 {
                    self.hit_points -= diff;
                }
            }
            DamageType::Spell => {
                let diff = damage - self.resilience;
                self.resilience = max(0, self.resilience - damage);
                if diff > 0 {
                    self.hit_points -= diff;
                }
            }
            DamageType::Chaos => {
                self.hit_points -= damage;
            }
        }
    }

    fn shuffle_deck(&mut self) {
        if let Some(deck) = &mut self.deck {
            if self.cards_in_discard.len() > 0 {
                deck.cards_in_deck.append(&mut self.cards_in_discard);
                self.cards_in_discard.clear();
            }
            use rand::seq::SliceRandom;
            use rand::thread_rng;

            let mut rng = thread_rng();
            deck.cards_in_deck.shuffle(&mut rng);
        }
    }

    fn draw_cards(&mut self, num_cards: i32) {
        // Ensure the deck exists and determine if we need to shuffle
        let need_shuffle = if let Some(deck) = &self.deck {
            deck.cards_in_deck.len() < num_cards as usize && !self.cards_in_discard.is_empty()
        } else {
            false
        };

        // If needed, shuffle the deck first
        if need_shuffle {
            self.shuffle_deck();
        }

        // After shuffling (if necessary), proceed with drawing cards
        if let Some(deck) = &mut self.deck {
            let cards_to_draw = num_cards.min(deck.cards_in_deck.len() as i32) as usize;
            let drawn_cards = deck
                .cards_in_deck
                .drain(0..cards_to_draw)
                .collect::<Vec<_>>();
            self.cards_in_hand.extend(drawn_cards);
        }
    }

    fn add_to_discard(&mut self, card: Card) {
        self.cards_in_discard.push(card);
    }
    fn add_mana(&mut self, mana: i32) {
        info!("NPC getting mana {:?}", mana);
        self.mana = mana;
    }

    fn spend_mana(&mut self, mana: i32) {
        self.mana -= mana;
    }
    fn get_hand(&self) -> &Vec<Card> {
        &self.cards_in_hand
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
            armor: data.armor,
            resilience: data.resilience,
            monster_type: None,
            mana: 0,
            deck: None,
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
