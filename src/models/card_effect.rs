use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// rename only when serializing to JSON
#[serde(rename_all = "camelCase")]
pub struct ActiveEffect {
    pub combatant_id: String, // Unique identifier for the combatant
    pub effect: ActiveEffectType,
    pub remaining_turns: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActiveEffectType {
    SkipTurn,
    Stun,
    Initiative { amount: i32 },
    Poison { amount: i32 },
    // Add other effects as needed
}
