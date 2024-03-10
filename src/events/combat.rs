use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use actix::Message;
use prisma_client_rust::chrono::{DateTime, Local};
use serde::{
    de::{EnumAccess, VariantAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde::ser::SerializeMap;
use serde_json::json;
use tracing::log::info;
use tracing::warn;

use crate::{
    models::{combatant::Combatant, hero::Hero, npc::Monster},
    services::impls::combat_service::CombatCommand,
};
use crate::events::combat::CombatantIndex::Combatant1;
use crate::models::cards::Card;
use crate::models::hero_combatant::HeroCombatant;
use crate::models::talent::{Effect, Talent};

// Damage over time
#[derive(Debug, Clone)]
pub struct Dot {
    name: String,
    damage_per_tick: i32,
    ticks_remaining: i32,
    target: CombatantIndex, // Our enhancement
}

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
    active_dots: Vec<Dot>,
    current_turn: CombatantIndex,
    status_effects: HashMap<CombatantId, Vec<Effect>>,
    started: bool,
    initial_hps: (i32, i32), // comb1 and comb2
}

impl CombatEncounter {
    pub fn new<T: Combatant + 'static>(hero: HeroCombatant, monster: T) -> Self {
        let hero_hp = hero.get_hp();
        let monster_hp = monster.get_hp();
        CombatEncounter {
            id: uuid::Uuid::new_v4().to_string(),
            combatant1: Arc::new(Mutex::new(hero)),
            combatant2: Arc::new(Mutex::new(monster)), // Box the generic monster
            battle_fields: HashMap::new(),
            active_dots: Vec::new(),
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
        index: CombatantIndex,
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

    fn apply_dots(&mut self) {
        for i in (0..self.active_dots.len()).rev() {
            // Iterate in reverse
            let dot = &mut self.active_dots[i];
            if dot.ticks_remaining > 0 {
                // ... your code to apply damage based on 'dot.target' ...
                dot.ticks_remaining -= 1;
            } else {
                self.active_dots.remove(i); // Remove directly in reverse order
            }
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
    fn perform_attack(&mut self, attacker: CombatantIndex) {
        let defender_guard = self.get_combatant(attacker.clone(), Some(true));
        let mut defender = defender_guard.lock().unwrap();
        let attacker_guard = self.get_combatant(attacker, None);
        let attacker = attacker_guard.lock().unwrap();
        //Damage Reduction % = Armor / Armor + (50 * EnemyLvl)
        let damage_reduction = defender.get_armor() as f32
            / (defender.get_armor() as f32 + (50.0 * defender.get_level() as f32));
        let attacker_damage = attacker.get_damage();
        let damage = (attacker_damage as f32 * (1.0 - damage_reduction)) as i32;

        info!(
            "{} attacks {} for {} damage with {} original damage, damage reduction: {}",
            attacker.get_name(),
            defender.get_name(),
            damage,
            attacker_damage,
            damage_reduction
        );
        defender.take_damage(damage);
        // defender.take_damage(damage);
    }

    fn get_current_turn(&self) -> CombatantIndex {
        self.current_turn.clone()
    }

    fn shuffle_cards(&mut self, combatant: CombatantIndex) {
        let combatant_arc_mutex = self.get_combatant(combatant, None); // This creates a longer-lived value
        let mut combatant = combatant_arc_mutex.lock().unwrap();
        combatant.shuffle_deck();
    }

    // shuffles deck and draws 5 cards
    pub fn initialize(&self, combatant_id: &str) -> anyhow::Result<()> {
        let combatant = self.get_combatant_by_id(combatant_id).unwrap();
        let mut locked = combatant.lock().unwrap();
        if locked.get_hand().is_empty() {
            locked.shuffle_deck();
            locked.draw_cards(5);
        }
        Ok(())
    }

    pub fn request_state(&self) -> EncounterState {
        EncounterState {
            combatant_1: self.combatant1.lock().unwrap().clone_box(),
            combatant_2: self.combatant2.lock().unwrap().clone_box(),
            battle_fields: self.battle_fields.clone(),
            turn: self.current_turn.clone(),
        }
    }

    pub fn process_combat_turn(
        &mut self,
        cmd: CombatCommand,
        combatant_id: &str, // the ID of the combatant making the move
    ) -> Result<CombatTurnMessage, CombatError> {
        use CombatCommand::*;
        let is_valid_turn = match self.current_turn {
            Combatant1 => combatant_id == self.combatant1.lock().unwrap().get_id(),
            CombatantIndex::Combatant2 => combatant_id == self.combatant2.lock().unwrap().get_id(),
        };
        if !is_valid_turn {
            return Err(CombatError::OutOfTurnAction);
        }
        let result = match cmd.clone() {
            Attack => {
                self.apply_dots();
                let current_attacker = self.get_current_turn();
                self.perform_attack(current_attacker.clone());
                let opponent = self
                    .get_combatant(current_attacker.clone(), Some(true))
                    .lock()
                    .unwrap()
                    .clone_box();
                if opponent.get_hp() <= 0 {
                    Ok(CombatTurnMessage::Winner(current_attacker))
                } else {
                    Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
                }
            }
            PlayCards(card) => {
                info!("Playing card: {:?}", card);
                // add logic here for card effects
                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            _ => {
                todo!()
            }
        };

        // 6. Toggle the current turn
        self.current_turn = match self.current_turn {
            Combatant1 => CombatantIndex::Combatant2,
            CombatantIndex::Combatant2 => Combatant1,
        };

        result // Return success on valid turn execution
    }
}

#[derive(Debug)]
pub enum CombatTurnMessage {
    CommandPlayed(CombatCommand),
    // Command played , result of the defender
    PlayerTurn(CombatantIndex),
    YourTurn(Box<dyn Combatant>), // only sent to the next turn player to show him cooldowns
    Winner(CombatantIndex),
    PlayerState {
        me: Box<dyn Combatant>,
        turn: CombatantIndex,
        my_battle_field: Vec<Card>,
        opponent_battle_field: Vec<Card>,
        opponent_hp: i32,
    },
    EncounterState(EncounterState), // Requested state
    EncounterStarted,
}

#[derive(Debug)]
pub struct EncounterState {
    pub turn: CombatantIndex,
    pub battle_fields: HashMap<CombatantIndex, Vec<Card>>,
    pub combatant_1: Box<dyn Combatant>,
    pub combatant_2: Box<dyn Combatant>,
}

impl Clone for CombatEncounter {
    fn clone(&self) -> Self {
        CombatEncounter {
            id: self.id.clone(),
            combatant1: self.combatant1.clone(),
            combatant2: self.combatant2.clone(),
            active_dots: self.active_dots.clone(),
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
            CombatTurnMessage::PlayerTurn(index) => CombatTurnMessage::PlayerTurn(index.clone()),
            CombatTurnMessage::Winner(idx) => CombatTurnMessage::Winner(idx.clone()),
            CombatTurnMessage::EncounterState(state) => {
                CombatTurnMessage::EncounterState(EncounterState {
                    turn: state.turn.clone(),
                    battle_fields: state.battle_fields.clone(),
                    combatant_1: state.combatant_1.clone_box(),
                    combatant_2: state.combatant_2.clone_box(),
                })
            }
            CombatTurnMessage::PlayerState {
                me,
                turn,
                my_battle_field,
                opponent_battle_field,
                opponent_hp,
            } => CombatTurnMessage::PlayerState {
                me: me.clone_box(),
                turn: turn.clone(),
                my_battle_field: my_battle_field.clone(),
                opponent_battle_field: opponent_battle_field.clone(),
                opponent_hp: *opponent_hp,
            },
            CombatTurnMessage::EncounterStarted => CombatTurnMessage::EncounterStarted,
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
                me,
                opponent_battle_field,
                opponent_hp,
                turn,
            } => {
                let mut map = serializer.serialize_map(None)?;
                let my_hand = me.get_hand();
                map.serialize_entry("type", "PlayerState")?;
                map.serialize_entry("turn", &turn);
                map.serialize_entry("my_battle_field", my_battle_field);
                map.serialize_entry("opponent_battle_field", opponent_battle_field);
                map.serialize_entry("hand", my_hand);
                map.serialize_entry("opponent_hp", opponent_hp);
                let me = json!({
                    "hp": me.get_hp(),
                    "name": me.get_name(),
                    "mana": me.get_mana(),
                });
                map.serialize_entry("me", &me);
                map.end()
            }
            CombatTurnMessage::EncounterStarted => serializer.serialize_str("EncounterStarted"),
            x => {
                warn!("Unimplemented serialization for {:?}", x);
                json!(null).serialize(serializer)
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
                    (Field::CommandPlayed, variant) => Ok(CombatTurnMessage::Winner(Combatant1)),
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
    // ... (Other error variants as needed) ...
}

impl CombatError {
    pub fn to_string(&self) -> String {
        match self {
            CombatError::OutOfTurnAction => "Out of turn action".to_string(),
            // ... (Other error variants as needed) ...
        }
    }
}
