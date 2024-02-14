use std::fmt;

use serde::{Deserialize, Serialize, Serializer, de::{Visitor, EnumAccess, VariantAccess}, Deserializer};

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

#[derive(Debug, Clone)]
pub enum CombatantIndex {
    Combatant1,
    Combatant2,
}

#[derive(Debug)]
pub struct CombatEncounter {
    id: String,
    combatant1: Box<dyn Combatant>,
    combatant2: Box<dyn Combatant>,
    active_dots: Vec<Dot>,
    current_turn: CombatantIndex,
}

impl CombatEncounter {
    // pub fn new(hero: Hero, monster: Box<dyn Combatant>) -> Self {
    pub fn new<T: Combatant + 'static>(hero: Hero, monster: T) -> Self {
        CombatEncounter {
            id: uuid::Uuid::new_v4().to_string(),
            combatant1: Box::new(hero),
            // combatant2: monster,
            combatant2: Box::new(monster), // Box the generic monster
            active_dots: Vec::new(),
            current_turn: CombatantIndex::Combatant1,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn has_combatant(&self, combatant_id: &str) -> bool {
        self.combatant1.get_id() == combatant_id || self.combatant2.get_id() == combatant_id
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

    pub fn process_combat_turn(
        &mut self,
        cmd: CombatCommand,
        combatant_id: &str,
    ) -> Result<CombatTurnResult, CombatError> {
        let is_valid_turn = match self.current_turn {
            CombatantIndex::Combatant1 => combatant_id == self.combatant1.get_id(),
            CombatantIndex::Combatant2 => combatant_id == self.combatant2.get_id(),
        };
        if !is_valid_turn {
            return Err(CombatError::OutOfTurnAction);
        }

        // 6. Toggle the current turn
        self.current_turn = match self.current_turn {
            CombatantIndex::Combatant1 => CombatantIndex::Combatant2,
            CombatantIndex::Combatant2 => CombatantIndex::Combatant1,
        };

        Ok(CombatTurnResult::Winner) // Return success on valid turn execution
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CombatantData {
    Monster(Monster),
    Hero(Hero),
}

#[derive(Debug)]
pub enum CombatTurnResult {
    CommandPlayed(Box<dyn Combatant>), // Command played , result of the defender
    Winner,                            // Potentially, if your rules allow for ties
}

impl Serialize for CombatTurnResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CombatTurnResult::CommandPlayed(combatant) => {
                let hp = combatant.get_hp();
                serializer.serialize_str("CommandPlayed")
            }
            CombatTurnResult::Winner => serializer.serialize_str("Winner"),
        }
    }
}

impl<'de> Deserialize<'de> for CombatTurnResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { CommandPlayed, Winner }

        struct CombatTurnResultVisitor;

        impl<'de> Visitor<'de> for CombatTurnResultVisitor {
            type Value = CombatTurnResult;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum CombatTurnResult")
            }

            fn visit_enum<A>(self, data: A) -> Result<CombatTurnResult, A::Error>
            where
                A: EnumAccess<'de>,
            {
                match data.variant()? {
                    (Field::CommandPlayed, variant) => {
                        let combatant_data: CombatantData = variant.newtype_variant()?;
                        match combatant_data {
                            CombatantData::Monster(monster) => Ok(CombatTurnResult::CommandPlayed(Box::new(monster))),
                            CombatantData::Hero(hero) => Ok(CombatTurnResult::CommandPlayed(Box::new(hero))),
                        }
                    },
                    (Field::Winner, _) => Ok(CombatTurnResult::Winner),
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["commandplayed", "winner"];
        deserializer.deserialize_enum("CombatTurnResult", FIELDS, CombatTurnResultVisitor)
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
