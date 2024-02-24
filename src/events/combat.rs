use actix::Message;
use prisma_client_rust::chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use serde::ser::SerializeMap;
use serde::{
    de::{EnumAccess, VariantAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use tracing::log::info;

use crate::events::combat::CombatantIndex::Combatant1;
use crate::models::talent::{Effect, Talent};
use crate::{
    models::{combatant::Combatant, hero::Hero, npc::Monster},
    services::impls::combat_service::CombatCommand,
};

// Damage over time
#[derive(Debug, Clone)]
pub struct Dot {
    name: String,
    damage_per_tick: i32,
    ticks_remaining: i32,
    target: CombatantIndex, // Our enhancement
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    active_dots: Vec<Dot>,
    current_turn: CombatantIndex,
    status_effects: HashMap<CombatantId, Vec<Effect>>,
    started: bool,
}

impl CombatEncounter {
    pub fn new<T: Combatant + 'static>(hero: Hero, monster: T) -> Self {
        CombatEncounter {
            id: uuid::Uuid::new_v4().to_string(),
            combatant1: Arc::new(Mutex::new(hero)),
            combatant2: Arc::new(Mutex::new(monster)), // Box the generic monster
            active_dots: Vec::new(),
            status_effects: HashMap::new(),
            current_turn: Combatant1,
            started: false,
        }
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
        let result = match cmd {
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
                    Ok(CombatTurnMessage::CommandPlayed(opponent))
                }
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
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CombatantData {
    Monster(Monster),
    Hero(Hero),
}

#[derive(Debug)]
pub enum CombatTurnMessage {
    CommandPlayed(Box<dyn Combatant>),
    // Command played , result of the defender
    PlayerTurn(CombatantIndex),
    YourTurn(Box<dyn Combatant>), // only sent to the next turn player to show him cooldowns
    Winner(CombatantIndex),
    // Potentially, if your rules allow for ties
    EncounterState(CombatEncounter),
    EncounterStarted,
}

impl Clone for CombatTurnMessage {
    fn clone(&self) -> Self {
        match self {
            CombatTurnMessage::CommandPlayed(combatant) => {
                CombatTurnMessage::CommandPlayed(combatant.clone_box())
            }
            CombatTurnMessage::YourTurn(combatant) => {
                CombatTurnMessage::YourTurn(combatant.clone_box())
            }
            CombatTurnMessage::PlayerTurn(index) => CombatTurnMessage::PlayerTurn(index.clone()),
            CombatTurnMessage::Winner(idx) => CombatTurnMessage::Winner(idx.clone()),
            CombatTurnMessage::EncounterState(encounter) => {
                CombatTurnMessage::EncounterState(encounter.clone())
            }
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
            CombatTurnMessage::CommandPlayed(combatant) => {
                let hp = combatant.get_hp();
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "CommandPlayed")?;
                map.serialize_entry("hp", &hp)?;
                map.serialize_entry("name", combatant.get_name())?;

                map.end()
            }
            CombatTurnMessage::Winner(idx) => {
                serializer.serialize_str(format!("Winner: {:?}", idx.clone()).as_str())
            }
            CombatTurnMessage::PlayerTurn(idx) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("PlayerTurn", &idx)?;
                map.end()
            }
            CombatTurnMessage::YourTurn(combatant) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", "YourTurn")?;
                map.serialize_entry("talents", &combatant.get_talents())?;
                map.end()
            }
            CombatTurnMessage::EncounterState(encounter) => {
                let mut map = serializer.serialize_map(Some(4))?;
                let mut combat1 = HashMap::new();
                let cb1 = encounter.combatant1.lock().unwrap();
                let cb2 = encounter.combatant2.lock().unwrap();
                let c1hp = cb1.get_hp().to_string();
                let c2hp = cb2.get_hp().to_string();
                combat1.insert("name", cb1.get_name());
                combat1.insert("hp", &c1hp);
                let mut combat2 = HashMap::new();
                combat2.insert("name", cb2.get_name());
                combat2.insert("hp", &c2hp);

                map.serialize_entry("Combatant1", &combat1)?;
                map.serialize_entry("Combatant2", &combat2)?;
                map.serialize_entry("turn", &encounter.current_turn)?;
                map.end()
            }
            CombatTurnMessage::EncounterStarted => serializer.serialize_str("EncounterStarted"),
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
                    (Field::CommandPlayed, variant) => {
                        let combatant_data: CombatantData = variant.newtype_variant()?;
                        match combatant_data {
                            CombatantData::Monster(monster) => {
                                Ok(CombatTurnMessage::CommandPlayed(Box::new(monster)))
                            }
                            CombatantData::Hero(hero) => {
                                Ok(CombatTurnMessage::CommandPlayed(Box::new(hero)))
                            }
                        }
                    }
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
