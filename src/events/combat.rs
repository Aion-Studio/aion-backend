use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

use crate::models::card_effect::ActiveEffect;
use crate::models::cards::Card;
use crate::models::combatant::CombatantType;
use crate::models::hero_combatant::HeroCombatant;
use crate::models::resources::Relic;
use crate::models::talent::{Effect, Spell};
use crate::{
    models::{combatant::Combatant, npc::Monster},
    services::impls::combat_service::CombatCommand,
};
use actix::Message;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::log::info;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CombatantIndex {
    Player,
    Npc,
}

use CombatantIndex::*;

#[derive(Serialize, Deserialize)]
pub struct SerializableCombatEncounter {
    id: String,
    player_combatant: HeroCombatant,
    npc_combatant: Monster,
    round: i32,
    action_id: Option<String>,
    active_effects: Vec<ActiveEffect>,
    current_turn: CombatantIndex,
    status_effects: HashMap<String, Vec<Effect>>,
    started: bool,
    initial_hps: (i32, i32),
}

trait CombatantExt {
    fn clone_as_hero_combatant(&self) -> HeroCombatant;
    fn clone_as_monster(&self) -> Monster;
}

impl CombatantExt for dyn Combatant {
    fn clone_as_hero_combatant(&self) -> HeroCombatant {
        if let Some(hero) = self.as_any().downcast_ref::<HeroCombatant>() {
            hero.clone()
        } else {
            panic!("Expected HeroCombatant")
        }
    }

    fn clone_as_monster(&self) -> Monster {
        if let Some(monster) = self.as_any().downcast_ref::<Monster>() {
            monster.clone()
        } else {
            panic!("Expected Monster")
        }
    }
}

type CombatantId = String;
#[derive(Debug, Serialize, Deserialize)]
pub struct CombatEncounter {
    id: String,
    pub player_combatant: CombatantType,
    pub npc_combatant: CombatantType,
    pub round: i32,
    pub action_id: Option<String>,
    active_effects: Vec<ActiveEffect>,
    current_turn: CombatantIndex,
    status_effects: HashMap<CombatantId, Vec<Effect>>,
    started: bool,
    initial_hps: (i32, i32), // comb1 and comb2
}

impl CombatEncounter {
    pub fn new(hero: HeroCombatant, npc: Monster) -> Self {
        let hero_hp = hero.get_hp();
        let monster_hp = npc.get_hp();
        let player_combatant = CombatantType::Hero(hero);
        let npc_combatant = CombatantType::Monster(npc);

        CombatEncounter {
            id: uuid::Uuid::new_v4().to_string(),
            player_combatant,
            npc_combatant, // Box the generic monster
            active_effects: Vec::new(),
            status_effects: HashMap::new(),
            current_turn: CombatantIndex::Player,
            started: false,
            initial_hps: (hero_hp, monster_hp),
            action_id: None,
            round: 1,
        }
    }
    pub fn set_action_id(&mut self, action_id: String) {
        self.action_id = Some(action_id);
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_combatant_ids(&self) -> Vec<String> {
        vec![
            self.player_combatant.as_combatant().get_id(),
            self.npc_combatant.as_combatant().get_id(),
        ]
    }

    pub fn has_combatant(&self, combatant_id: &str) -> bool {
        self.player_combatant.as_combatant().get_id() == combatant_id
            || self.npc_combatant.as_combatant().get_id() == combatant_id
    }

    pub fn whos_turn(&self) -> CombatantIndex {
        self.current_turn.clone()
    }
    pub fn get_combatant(
        &self,
        index: &CombatantIndex,
        is_opponent: Option<bool>,
    ) -> CombatantType {
        match index {
            Player => {
                if is_opponent.unwrap_or(false) {
                    self.npc_combatant.clone()
                } else {
                    self.player_combatant.clone()
                }
            }
            Npc => {
                if is_opponent.unwrap_or(false) {
                    self.player_combatant.clone()
                } else {
                    self.npc_combatant.clone()
                }
            }
        }
    }

    pub fn get_combatant_by_id(&self, combatant_id: &str) -> Option<CombatantType> {
        let player_combatant_id = self.player_combatant.as_combatant().get_id();
        if combatant_id == player_combatant_id {
            Some(self.player_combatant.clone())
        } else {
            Some(self.npc_combatant.clone())
        }
    }

    pub fn get_opponent(&self, combatant_id: &str) -> CombatantType {
        let player_combatant_id = self.player_combatant.as_combatant().get_id();
        if combatant_id == player_combatant_id {
            self.npc_combatant.clone()
        } else {
            self.player_combatant.clone()
        }
    }

    pub fn get_combatant_idx(&self, combatant_id: &str) -> Option<CombatantIndex> {
        if combatant_id == self.player_combatant.as_combatant().get_id() {
            Some(Player)
        } else if combatant_id == self.npc_combatant.as_combatant().get_id() {
            Some(Npc)
        } else {
            None
        }
    }

    // shuffles deck and draws 5 cards
    pub fn initialize(&mut self, combatant_id: &str) -> anyhow::Result<()> {
        let mut combatant_type = self.get_combatant(&Player, None);
        let combatant = combatant_type.as_combatant_mut();

        if combatant.get_hand().is_empty() {
            combatant.shuffle_deck();
            combatant.draw_cards();
        }
        Ok(())
    }

    // fn handle_attack(
    //     &mut self,
    //     attacker_idx: CombatantIndex,
    //     attacker_id: &str,
    //     target_minion_id: &str,
    // ) -> Result<(i32, i32), CombatError> {
    //     let defender_idx = match attacker_idx {
    //         player_combatant => CombatantIndex::npc_combatant,
    //         CombatantIndex::npc_combatant => player_combatant,
    //     };
    //     // Step 1: Grab and clone the cards from the vectors
    //     let (mut attacker_card, mut defender_card) = {
    //         let attacker_card = self
    //             .find_and_clone_card(&attacker_idx, attacker_id)
    //             .ok_or(CombatError::CardNotFound)?;
    //
    //         let defender_card = self
    //             .find_and_clone_card(&defender_idx, target_minion_id)
    //             .ok_or(CombatError::CardNotFound)?;
    //
    //         (attacker_card, defender_card)
    //     };
    //
    //     // saving the numbers before actual attack
    //     let defender_damage_taken = (attacker_card.damage).max(0);
    //     let attacker_damage_taken = (defender_card.damage).max(0);
    //
    //     // Step 3: Perform the attack logic on the cloned cards
    //     // Assume attack modifies the cards in place;
    //     attacker_card.attack(&mut defender_card);
    //     // max of 0 and the difference between the health and the damage
    //
    //     if defender_card.health <= 0 {
    //         self.remove_card(&defender_idx, target_minion_id);
    //     } else {
    //         self.update_card_in_battle_field(&defender_idx, &defender_card)?;
    //         // If the defender survives, it counterattacks
    //         defender_card.attack(&mut attacker_card);
    //         self.update_card_in_battle_field(&attacker_idx, &attacker_card)?;
    //         if attacker_card.health <= 0 {
    //             self.remove_card(&attacker_idx, attacker_id);
    //         }
    //     }
    //
    //     Ok((attacker_damage_taken, defender_damage_taken))
    // }

    // Utility function to find a card by ID and clone it

    pub fn process_combat_turn(
        &mut self,
        cmd: CombatCommand,
        combatant_id: &str, // the ID of the combatant making the move
    ) -> Result<CombatTurnMessage, CombatError> {
        use CombatCommand::*;
        let (idx, opponent_idx) = match self.get_combatant_idx(combatant_id) {
            Some(Player) => (Player, Npc),
            Some(Npc) => (Npc, Player),
            None => return Err(CombatError::CardNotFound),
        };
        let is_valid_turn = self.current_turn == idx;
        if !is_valid_turn {
            return Err(CombatError::OutOfTurnAction);
        }
        let combatant = self
            .get_combatant_by_id(combatant_id)
            .unwrap()
            .as_combatant();
        let result = match cmd.clone() {
            AttackHero(card) => {
                // TODO: FIll in
                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            PlayCard(mut card) => {
                // TODO FIll in
                todo!();

                drop(combatant);
                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            EndTurn => {
                self.current_turn = match idx {
                    Player => CombatantIndex::Npc,
                    Npc => Player,
                };

                // increment round draws cards for combatant 1, else statement draws for combatant 2
                if self.current_turn == Player {
                    self.increment_round();
                } else {
                    let mut combatant_type = self.get_combatant(&CombatantIndex::Npc, None);
                    let combatant = combatant_type.as_combatant_mut();
                    combatant.draw_cards();
                }

                if let Some(_) = self.check_skip_turn_effects() {
                    return Ok(CombatTurnMessage::PlayerMissesTurn);
                }

                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            _ => {
                todo!()
            }
        };

        result
    }

    fn check_skip_turn_effects(&mut self) -> Option<()> {
        unimplemented!()
    }

    fn increment_round(&mut self) {
        info!("Ending Round");
        self.round += 1;
        {
            let player_combatant = self.player_combatant.as_combatant_mut();
            player_combatant.add_mana();
            player_combatant.draw_cards();
        }
        {
            let npc_combatant = self.npc_combatant.as_combatant_mut();
            npc_combatant.add_mana();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerCombatState {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CombatTurnMessage {
    CommandPlayed(CombatCommand),
    // Command played , result of the defender
    PlayerTurn(CombatantIndex),
    PlayerMissesTurn,
    Winner(CombatantIndex),
    PlayerState(CombatantState),
    EncounterData(EncounterState), // Requested state
    Temp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CombatantState {
    Player {
        max_hp: i32,
        hp: i32,
        mana: i32,
        armor: i32,
        zeal: i32,
        strength: i32,
        intelligence: i32,
        dexterity: i32,
        spells: Vec<Spell>,
        relics: Vec<Relic>,
        drawn_cards: Vec<Card>,
        cards_in_discard: Vec<Card>,
    },
    Npc {
        max_hp: i32,
        hp: i32,
        spells: Vec<Spell>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncounterState {
    pub turn: CombatantIndex,
    pub round: i32,
    pub player_state: CombatantState,
    pub npc_state: CombatantState,
}

impl Clone for CombatEncounter {
    fn clone(&self) -> Self {
        CombatEncounter {
            id: self.id.clone(),
            player_combatant: self.player_combatant.clone(),
            npc_combatant: self.npc_combatant.clone(),
            active_effects: self.active_effects.clone(),
            current_turn: self.current_turn.clone(),
            started: self.started,
            status_effects: self.status_effects.clone(),
            initial_hps: self.initial_hps,
            action_id: self.action_id.clone(),
            round: self.round,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CombatantData {
    Monster(Monster),
    Hero(HeroCombatant),
}

impl Message for CombatTurnMessage {
    type Result = ();
}

#[derive(Debug)]
pub enum CombatError {
    OutOfTurnAction,
    ManaError,
    JustPlayedCardError,
    CardNotInHand, // ... (Other error variants as needed) ...
    CardNotFound,
}

impl Display for CombatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatError::OutOfTurnAction => write!(f, "Out of turn action"),
            CombatError::ManaError => write!(f, "Not enough mana"),
            CombatError::CardNotInHand => write!(f, "Card not in hand"),
            CombatError::CardNotFound => write!(f, "Card not found"),
            CombatError::JustPlayedCardError => write!(f, "Just played card"),
        }
    }
}
