use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

use crate::events::combat::CombatantIndex::Combatant1;
use crate::models::card_effect::{ActiveEffect, ActiveEffectType};
use crate::models::cards::{Card, CardType};
use crate::models::hero_combatant::HeroCombatant;
use crate::models::talent::Effect;
use crate::{
    models::{combatant::Combatant, npc::Monster},
    services::impls::combat_service::CombatCommand,
};
use actix::Message;
use serde::ser::SerializeMap;
use serde::{
    de::{EnumAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::json;
use tracing::log::info;
use tracing::{error, warn};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CombatantIndex {
    Combatant1,
    Combatant2,
}
type CombatantId = String;
#[derive(Debug)]
pub struct CombatEncounter {
    id: String,
    pub combatant1: Arc<Mutex<dyn Combatant>>,
    pub combatant2: Arc<Mutex<dyn Combatant>>,
    pub battle_fields: HashMap<CombatantIndex, Vec<Card>>,
    pub round: i32,
    pub action_id: Option<String>,
    active_effects: Vec<ActiveEffect>,
    current_turn: CombatantIndex,
    status_effects: HashMap<CombatantId, Vec<Effect>>,
    started: bool,
    initial_hps: (i32, i32), // comb1 and comb2
}

impl CombatEncounter {
    pub fn new<T: Combatant + 'static>(hero: HeroCombatant, monster: T) -> Self {
        let hero_hp = hero.get_hp();
        let monster_hp = monster.get_hp();
        let combatant1 = Arc::new(Mutex::new(hero));
        let combatant2 = Arc::new(Mutex::new(monster));
        combatant1.lock().unwrap().add_mana(1);
        combatant2.lock().unwrap().add_mana(1);

        CombatEncounter {
            id: uuid::Uuid::new_v4().to_string(),
            combatant1,
            combatant2, // Box the generic monster
            battle_fields: HashMap::new(),
            active_effects: Vec::new(),
            status_effects: HashMap::new(),
            current_turn: Combatant1,
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
            self.combatant1.lock().unwrap().get_id(),
            self.combatant2.lock().unwrap().get_id(),
        ]
    }

    pub fn has_combatant(&self, combatant_id: &str) -> bool {
        self.combatant1.lock().unwrap().get_id() == combatant_id
            || self.combatant2.lock().unwrap().get_id() == combatant_id
    }

    pub fn whos_turn(&self) -> CombatantIndex {
        self.current_turn.clone()
    }
    pub fn get_combatant(
        &self,
        index: &CombatantIndex,
        is_opponent: Option<bool>,
    ) -> Arc<Mutex<dyn Combatant>> {
        match index {
            Combatant1 => {
                if is_opponent.unwrap_or(false) {
                    self.combatant2.clone()
                } else {
                    self.combatant1.clone()
                }
            }
            CombatantIndex::Combatant2 => {
                if is_opponent.unwrap_or(false) {
                    self.combatant1.clone()
                } else {
                    self.combatant2.clone()
                }
            }
        }
    }

    pub fn get_combatant_by_id(&self, combatant_id: &str) -> Option<Arc<Mutex<dyn Combatant>>> {
        let combatant1_id = self.combatant1.lock().unwrap().get_id();
        if combatant_id == combatant1_id {
            Some(self.combatant1.clone())
        } else {
            Some(self.combatant2.clone())
        }
    }

    pub fn get_opponent(&self, combatant_id: &str) -> Arc<Mutex<dyn Combatant>> {
        let combatant1_id = self.combatant1.lock().unwrap().get_id();
        if combatant_id == combatant1_id {
            self.combatant2.clone()
        } else {
            self.combatant1.clone()
        }
    }

    pub fn get_combatant_idx(&self, combatant_id: &str) -> Option<CombatantIndex> {
        if combatant_id == self.combatant1.lock().unwrap().get_id() {
            Some(Combatant1)
        } else if combatant_id == self.combatant2.lock().unwrap().get_id() {
            Some(CombatantIndex::Combatant2)
        } else {
            None
        }
    }

    // shuffles deck and draws 5 cards
    pub fn initialize(&mut self, combatant_id: &str) -> anyhow::Result<()> {
        let combatant_idx = self.get_combatant_idx(combatant_id).unwrap();
        let opponent_idx = match combatant_idx {
            Combatant1 => CombatantIndex::Combatant2,
            CombatantIndex::Combatant2 => Combatant1,
        };

        for idx in [combatant_idx, opponent_idx] {
            self.battle_fields.insert(idx.clone(), vec![]); // Initialize battle fields
            let combatant = self.get_combatant(&idx, None);
            let mut locked = combatant.lock().unwrap();
            let num_cards = if self.current_turn == idx { 6 } else { 5 };

            if locked.get_hand().is_empty() {
                locked.shuffle_deck();
                locked.draw_cards(num_cards);
            }
        }
        Ok(())
    }

    pub fn request_state(&self) -> EncounterState {
        EncounterState {
            combatant_1: self.combatant1.lock().unwrap().clone_box(),
            combatant_2: self.combatant2.lock().unwrap().clone_box(),
            battle_fields: self.battle_fields.clone(),
            turn: self.current_turn.clone(),
            round: self.round,
            active_effects: self.active_effects.clone(),
        }
    }
    fn handle_attack(
        &mut self,
        attacker_idx: CombatantIndex,
        attacker_id: &str,
        target_minion_id: &str,
    ) -> Result<(i32, i32), CombatError> {
        let defender_idx = match attacker_idx {
            Combatant1 => CombatantIndex::Combatant2,
            CombatantIndex::Combatant2 => Combatant1,
        };
        // Step 1: Grab and clone the cards from the vectors
        let (mut attacker_card, mut defender_card) = {
            let attacker_card = self
                .find_and_clone_card(&attacker_idx, attacker_id)
                .ok_or(CombatError::CardNotFound)?;

            let defender_card = self
                .find_and_clone_card(&defender_idx, target_minion_id)
                .ok_or(CombatError::CardNotFound)?;

            (attacker_card, defender_card)
        };

        // saving the numbers before actual attack
        let defender_damage_taken = (attacker_card.damage).max(0);
        let attacker_damage_taken = (defender_card.damage).max(0);

        // Step 3: Perform the attack logic on the cloned cards
        // Assume attack modifies the cards in place;
        attacker_card.attack(&mut defender_card);
        // max of 0 and the difference between the health and the damage

        if defender_card.health <= 0 {
            self.remove_card(&defender_idx, target_minion_id);
        } else {
            self.update_card_in_battle_field(&defender_idx, &defender_card)?;
            // If the defender survives, it counterattacks
            defender_card.attack(&mut attacker_card);
            self.update_card_in_battle_field(&attacker_idx, &attacker_card)?;
            if attacker_card.health <= 0 {
                self.remove_card(&attacker_idx, attacker_id);
            }
        }

        Ok((attacker_damage_taken, defender_damage_taken))
    }

    // Utility function to find a card by ID and clone it
    fn find_and_clone_card(&self, idx: &CombatantIndex, card_id: &str) -> Option<Card> {
        self.battle_fields
            .get(idx)?
            .iter()
            .find(|card| card.id == card_id)
            .cloned() // Clones the found card
    }

    // Utility function to update a card in the battle field
    // This replaces the card with a new version
    fn update_card_in_battle_field(
        &mut self,
        idx: &CombatantIndex,
        updated_card: &Card,
    ) -> Result<(), CombatError> {
        let cards = self
            .battle_fields
            .get_mut(idx)
            .ok_or(CombatError::CardNotFound)?;
        if let Some(pos) = cards.iter().position(|card| card.id == updated_card.id) {
            cards[pos] = updated_card.clone(); // Replace the old card with the updated one
        } else {
            return Err(CombatError::CardNotFound);
        }
        Ok(())
    }

    fn remove_card(&mut self, idx: &CombatantIndex, card_id: &str) {
        let card_option = {
            // Directly remove the card from the battle_fields by iterating with enumeration, which allows us to remove by index
            let cards = self.battle_fields.get_mut(idx).unwrap(); // Safely unwrapped assuming idx is always valid
            let card_pos = cards.iter().position(|card| card.id == card_id);
            card_pos.map(|pos| cards.remove(pos)) // Remove the card at the found position, returns Option<Card>
        };

        if let Some(card) = card_option {
            // If the card was found and removed, now add it to the combatant's discard pile
            let combatant = self.get_combatant(idx, None);
            let mut guard = combatant.lock().unwrap();
            guard.add_to_discard(card);
        } else {
            // Handle the error case where the card was not found
            error!("Card not found but tried to remove from battle field");
            return;
        }
    }

    pub fn process_combat_turn(
        &mut self,
        cmd: CombatCommand,
        combatant_id: &str, // the ID of the combatant making the move
    ) -> Result<CombatTurnMessage, CombatError> {
        use CombatCommand::*;
        let idx = self.get_combatant_idx(combatant_id).unwrap();
        let opponent_idx = match idx {
            Combatant1 => CombatantIndex::Combatant2,
            CombatantIndex::Combatant2 => Combatant1,
        };
        let is_valid_turn = self.current_turn == idx;
        if !is_valid_turn {
            return Err(CombatError::OutOfTurnAction);
        }
        let arc = self.get_combatant_by_id(combatant_id).unwrap();
        let result = match cmd.clone() {
            AttackMinion {
                attacker,
                defender_id,
            } => {
                let attacker_card = self
                    .battle_fields
                    .get_mut(&idx)
                    .unwrap()
                    .iter_mut()
                    .find(|c| c.id == attacker.id)
                    .unwrap();

                info!(
                    "Round played check: curr round {:?} attacker played : {:?}",
                    self.round, attacker_card.round_played
                );
                if attacker_card.round_played == self.round
                    || attacker_card.last_attack_round == Some(self.round)
                {
                    warn!("cant attack with just played card");
                    return Err(CombatError::JustPlayedCardError);
                }
                attacker_card.last_attack_round = Some(self.round);
                self.handle_attack(idx, &attacker.id, &defender_id).map(
                    |(attacker_damage_taken, defender_damage_taken)| {
                        CombatTurnMessage::CommandPlayed(AttackExchange {
                            attacker_damage_taken,
                            defender_damage_taken,
                            attacker_id: attacker.id,
                            defender_id,
                        })
                    },
                )
            }
            AttackHero(card) => {
                let can_play = {
                    let attacker_card = self
                        .battle_fields
                        .get_mut(&idx)
                        .unwrap()
                        .iter_mut()
                        .find(|c| c.id == card.id)
                        .unwrap();
                    if attacker_card.round_played == self.round {
                        false
                    } else {
                        true
                    }
                };

                if !can_play {
                    return Err(CombatError::JustPlayedCardError);
                }

                let opponent = self.get_combatant(&idx, Some(true));
                info!("attacking with card damanage {:?}", card.damage);
                let mut opponent = opponent.lock().unwrap();
                opponent.take_damage(card.damage, DamageType::Physical);
                let attacker_card = self
                    .battle_fields
                    .get_mut(&idx)
                    .unwrap()
                    .iter_mut()
                    .find(|c| c.id == card.id)
                    .unwrap();

                attacker_card.last_attack_round = Some(self.round);
                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            PlayCard(mut card) => {
                let mut combatant = arc.lock().unwrap();

                if combatant.get_mana() < card.mana_cost {
                    return Err(CombatError::ManaError);
                }
                card.round_played = self.round;

                combatant.play_card(&card)?;
                combatant.spend_mana(card.mana_cost);

                if card.card_type == CardType::Spell {
                    self.apply_spell_effects(&card, &idx);
                    self.battle_fields
                        .get_mut(&idx)
                        .unwrap()
                        .retain(|c| c.id != card.id);
                    combatant.add_to_discard(card);
                } else {
                    info!(
                        "zzzzz updating battle field for card played {:?} and idx {:?}",
                        card, idx
                    );
                    self.battle_fields.get_mut(&idx).unwrap().push(card);
                }

                drop(combatant);
                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            EndTurn => {
                self.current_turn = match self.current_turn {
                    Combatant1 => CombatantIndex::Combatant2,
                    CombatantIndex::Combatant2 => Combatant1,
                };

                // increment round draws cards for combatant 1, else statement draws for combatant 2
                if self.current_turn == Combatant1 {
                    self.increment_round();
                } else {
                    let mutex = self.get_combatant(&CombatantIndex::Combatant2, None);
                    let mut combatant = mutex.lock().unwrap();
                    info!(
                        "----drawing 1 card for combatant {:?}",
                        combatant.get_name()
                    );
                    combatant.draw_cards(1);
                }

                if let Some(_) = self.check_skip_turn_effects() {
                    return Ok(CombatTurnMessage::PlayerMissesTurn);
                }

                self.apply_dots();

                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            _ => {
                todo!()
            }
        };

        result
    }

    /// Applies damage over time to opponent on your turn
    fn apply_dots(&self) {
        let turn = match self.current_turn {
            Combatant1 => CombatantIndex::Combatant2,
            CombatantIndex::Combatant2 => Combatant1,
        };
        for active_effect in &self.active_effects {
            if active_effect.combatant_id
                == self.get_combatant(&turn, None).lock().unwrap().get_id()
            {
                match active_effect.effect {
                    ActiveEffectType::Poison { amount } => {
                        let combatant = self.get_combatant(&turn, None);
                        let mut combatant = combatant.lock().unwrap();
                        combatant.take_damage(amount, DamageType::Chaos);
                    }
                    _ => {}
                }
            }
        }
    }

    fn check_skip_turn_effects(&mut self) -> Option<()> {
        let turn = self.current_turn.clone();
        let mut to_remove = vec![];
        let mut result = None;
        for (index, effect) in self.active_effects.iter().enumerate() {
            if effect.combatant_id == self.get_combatant(&turn, None).lock().unwrap().get_id() {
                match effect.effect {
                    ActiveEffectType::Initiative { amount } => {
                        if amount == 3 {
                            self.current_turn = match turn {
                                CombatantIndex::Combatant1 => CombatantIndex::Combatant2,
                                CombatantIndex::Combatant2 => CombatantIndex::Combatant1,
                            };
                            to_remove.push(index); // Mark this effect for removal
                            result = Some(());
                        }
                    }
                    ActiveEffectType::Stun => {
                        self.current_turn = match turn {
                            CombatantIndex::Combatant1 => CombatantIndex::Combatant2,
                            CombatantIndex::Combatant2 => CombatantIndex::Combatant1,
                        };
                        to_remove.push(index); // Mark this effect for remova
                                               //
                        result = Some(());
                    }
                    _ => {}
                }
            }
        }
        for index in to_remove.into_iter().rev() {
            self.active_effects.swap_remove(index);
        }
        result
    }

    fn increment_round(&mut self) {
        info!("Ending Round");
        self.round += 1;
        {
            let mut combatant1 = self.combatant1.lock().unwrap();
            combatant1.add_mana(self.round);
            combatant1.draw_cards(1);
        }
        {
            let mut combatant2 = self.combatant2.lock().unwrap();
            combatant2.add_mana(self.round);
        }
    }
}

#[derive(Debug)]
pub enum CombatTurnMessage {
    CommandPlayed(CombatCommand),
    // Command played , result of the defender
    PlayerTurn(CombatantIndex),
    PlayerMissesTurn,
    YourTurn(Box<dyn Combatant>), // only sent to the next turn player to show him cooldowns
    Winner(CombatantIndex),
    PlayerState {
        me: Box<dyn Combatant>,
        me_idx: CombatantIndex,
        turn: CombatantIndex,
        my_battle_field: Vec<Card>,
        opponent_battle_field: Vec<Card>,
        opponent: Box<dyn Combatant>,
        opponent_hp: i32,
        active_effects: Vec<ActiveEffect>,
    },
    EncounterState(EncounterState), // Requested state
}

#[derive(Debug)]
pub struct EncounterState {
    pub turn: CombatantIndex,
    pub round: i32,
    pub battle_fields: HashMap<CombatantIndex, Vec<Card>>,
    pub combatant_1: Box<dyn Combatant>,
    pub combatant_2: Box<dyn Combatant>,
    pub active_effects: Vec<ActiveEffect>,
}

impl Clone for CombatEncounter {
    fn clone(&self) -> Self {
        CombatEncounter {
            id: self.id.clone(),
            combatant1: self.combatant1.clone(),
            combatant2: self.combatant2.clone(),
            active_effects: self.active_effects.clone(),
            current_turn: self.current_turn.clone(),
            started: self.started,
            status_effects: self.status_effects.clone(),
            initial_hps: self.initial_hps,
            action_id: self.action_id.clone(),
            round: self.round,
            battle_fields: self.battle_fields.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CombatantData {
    Monster(Monster),
    Hero(HeroCombatant),
}

impl Clone for CombatTurnMessage {
    fn clone(&self) -> Self {
        match self {
            CombatTurnMessage::CommandPlayed(cmd) => CombatTurnMessage::CommandPlayed(cmd.clone()),
            CombatTurnMessage::YourTurn(combatant) => {
                CombatTurnMessage::YourTurn(combatant.clone_box())
            }
            CombatTurnMessage::PlayerMissesTurn => CombatTurnMessage::PlayerMissesTurn,
            CombatTurnMessage::PlayerTurn(index) => CombatTurnMessage::PlayerTurn(index.clone()),
            CombatTurnMessage::Winner(idx) => CombatTurnMessage::Winner(idx.clone()),
            CombatTurnMessage::EncounterState(state) => {
                CombatTurnMessage::EncounterState(EncounterState {
                    round: state.round,
                    turn: state.turn.clone(),
                    battle_fields: state.battle_fields.clone(),
                    combatant_1: state.combatant_1.clone_box(),
                    combatant_2: state.combatant_2.clone_box(),
                    active_effects: state.active_effects.clone(),
                })
            }
            CombatTurnMessage::PlayerState {
                me,
                me_idx,
                turn,
                my_battle_field,
                active_effects,
                opponent_battle_field,
                opponent,
                opponent_hp,
            } => CombatTurnMessage::PlayerState {
                me: me.clone_box(),
                me_idx: me_idx.clone(),
                opponent: opponent.clone_box(),
                turn: turn.clone(),
                active_effects: active_effects.clone(),
                my_battle_field: my_battle_field.clone(),
                opponent_battle_field: opponent_battle_field.clone(),
                opponent_hp: *opponent_hp,
            },
        }
    }
}

impl Message for CombatTurnMessage {
    type Result = ();
}

impl Serialize for CombatTurnMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CombatTurnMessage::CommandPlayed(cmd) => {
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "CommandPlayed")?;
                map.serialize_entry("value", &json!(cmd))?;
                map.end()
            }
            CombatTurnMessage::Winner(idx) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "Winner")?;
                map.serialize_entry("value", &idx)?;
                map.end()
            }
            CombatTurnMessage::PlayerTurn(idx) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "PlayerTurn")?;
                map.serialize_entry("value", &idx)?;
                map.end()
            }
            CombatTurnMessage::YourTurn(combatant) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", "YourTurn")?;
                map.serialize_entry("talents", &combatant.get_talents())?;
                map.end()
            }
            CombatTurnMessage::PlayerState {
                my_battle_field,
                me_idx,
                me,
                opponent_battle_field,
                opponent_hp,
                opponent,
                active_effects,
                turn,
            } => {
                let mut map = serializer.serialize_map(None)?;
                let my_hand = me.get_hand();
                map.serialize_entry("type", "PlayerState")?;
                map.serialize_entry("turn", &turn)
                    .expect("TODO: panic message");
                map.serialize_entry("my_battle_field", my_battle_field)
                    .expect("TODO: panic message");
                map.serialize_entry("opponent_battle_field", opponent_battle_field)
                    .expect("TODO: panic message");
                map.serialize_entry("hand", my_hand)
                    .expect("TODO: panic message");
                map.serialize_entry("opponent_hp", opponent_hp)
                    .expect("TODO: panic message");
                map.serialize_entry("active_effects", active_effects)
                    .expect("TODO: panic message");
                let me = json!({
                    "hp": me.get_hp(),
                    "name": me.get_name(),
                    "mana": me.get_mana(),
                    "armor": me.get_armor(),
                    "idx": me_idx
                });
                let oppo = json!({
                    "hp": opponent_hp,
                    "name": opponent.get_name(),
                    "mana": opponent.get_mana(),
                    "armor": opponent.get_armor(),
                });
                map.serialize_entry("opponent", &oppo)
                    .expect("TODO: panic message");
                map.serialize_entry("me", &me).expect("TODO: panic message");
                map.end()
            }
            CombatTurnMessage::PlayerMissesTurn => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "PlayerMissesTurn")?;
                map.end()
            }
            _ => {
                warn!("attempted to serialize a message not meant to be sent to the client");
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "Invalid")?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for CombatTurnMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            CommandPlayed,
            Winner,
        }

        struct CombatTurnMessageVisitor;

        impl<'de> Visitor<'de> for CombatTurnMessageVisitor {
            type Value = CombatTurnMessage;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum CombatTurnMessage")
            }

            fn visit_enum<A>(self, data: A) -> Result<CombatTurnMessage, A::Error>
            where
                A: EnumAccess<'de>,
            {
                match data.variant()? {
                    (Field::CommandPlayed, _) => Ok(CombatTurnMessage::Winner(Combatant1)),
                    (Field::Winner, _) => Ok(CombatTurnMessage::Winner(Combatant1)),
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["commandplayed", "winner"];
        deserializer.deserialize_enum("CombatTurnMessage", FIELDS, CombatTurnMessageVisitor)
    }
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

// `?` couldn't convert the error to `events::combat::CombatError` [E0277] Note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait Help: the following other types implement trait `std::ops::FromResidual<R>`: <std::result::Result<T, F> as std::ops::FromResidual<std::ops::Yeet<E>>> <std::result::Result<T, F> as std::ops::FromResidual<std::result::Result<std::convert::Infallible, E>>> Note: required for `std::result::Result<events::combat::CombatTurnMessage, events::combat::CombatError>` to implement `std::ops::FromResidual<std::result::Result<std::convert::Infallible, anyhow::Error>>`
