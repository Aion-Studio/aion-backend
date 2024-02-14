use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::error;

use crate::prisma::{
    follower_talent, hero_talent,
    talent::{self},
};

use super::{hero::BaseStats, resources::Resource};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Talent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cooldown: i32, // Turns remaining until usable again
    pub effects: Vec<Effect>,
}

impl From<hero_talent::Data> for Talent {
    fn from(data: hero_talent::Data) -> Self {
        let talent = match data.talent {
            Some(talent) => talent,
            None => Box::new(talent::Data {
                hero_talents: None,
                follower_talents: None,

                id: "".to_string(),
                name: "".to_string(),
                description: Some("".to_string()),
                cooldown: 0,
                effects: json!({}),
            }),
        };
        Talent {
            id: talent.id,
            name: talent.name,
            description: match talent.description {
                Some(description) => description,
                None => "".to_string(),
            },
            cooldown: talent.cooldown,
            effects: match effects_from_json(&talent.effects) {
                Ok(effects) => effects,
                Err(e) => {
                    error!("Error deserializing talent effects: {}", e);
                    vec![]
                }
            },
        }
    }
}

impl From<follower_talent::Data> for Talent {
    fn from(data: follower_talent::Data) -> Self {
        let talent = match data.talent {
            Some(talent) => talent,
            None => Box::new(talent::Data {
                hero_talents: None,
                follower_talents: None,

                id: "".to_string(),
                name: "".to_string(),
                description: Some("".to_string()),
                cooldown: 0,
                effects: json!({}),
            }),
        };
        Talent {
            id: talent.id,
            name: talent.name,
            description: match talent.description {
                Some(description) => description,
                None => "".to_string(),
            },
            cooldown: talent.cooldown,
            effects: match effects_from_json(&talent.effects) {
                Ok(effects) => effects,
                Err(e) => {
                    error!("Error deserializing talent effects: {}", e);
                    vec![]
                }
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Effect {
    // Damage Effects
    Damage(i32),              // Amount of damage to inflict immediately
    DamageOverTime(i32, i32), // Damage to deal per turn, number of turns to apply the damage
    PercentageDamage(f32),    // Percentage of the target's maximum health to inflict as damage

    // Stat Manipulation Effects
    ModifyStat(BaseStats, i32, i32), // The stat to modify, amount to change (positive or negative), duration in turns

    // Crowd Control Effects
    Stun,    // Enemy misses their next turn
    Silence, // Enemy cannot use abilities for their next turn
    Disarm,  // Enemy cannot use basic attacks for their next turn

    // Healing Effects
    Heal(i32),              // Amount of health to restore immediately
    HealOverTime(i32, i32), // Amount of health to restore per turn, duration in turns

    // Resource Manipulation Effects
    DrainResource(Resource, i32), // Type of resource to drain, amount to drain
    RestoreResource(Resource, i32), // Type of resource to restore, amount to restore

    // Utility Effects
    CooldownReset(String), // The name or ID of the ability whose cooldown to reset
    TriggerEffect(Box<Effect>), // Another effect to immediately trigger (for chaining)
                           //
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CombatAction {
    pub name: String,         // Name of the talent or follower ability
    pub source: ActionSource, // Whether the action comes from a talent or follower
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ActionSource {
    Talent,
    Follower,
}

// fn serialize_effect(effect: &Effect) -> Value {
//     match effect {
//         Effect::Damage(amount) => json!({"type": "Damage", "amount": amount}),
//         // Add cases for other variants
//         _ => json!({}), // Placeholder for unimplemented variants
//     }
// }
//
// // Function to deserialize JSON to an Effect (outline)
// fn deserialize_effect(value: &Value) -> Result<Effect, String> {
//     let obj = value.as_object().ok_or("Expected a JSON object")?;
//     match obj.get("type").and_then(Value::as_str) {
//         Some("Damage") => {
//             let amount = obj
//                 .get("amount")
//                 .and_then(Value::as_i64)
//                 .ok_or("Expected amount")?;
//             Ok(Effect::Damage(amount as i32))
//         }
//         // Add cases for other variants
//         _ => Err("Unknown effect type".to_string()),
//     }
// }

// Effect -> JSON
fn effect_to_json(effect: &Effect) -> Value {
    match effect {
        Effect::Damage(amount) => json!({ "type": "Damage", "amount": amount }),
        Effect::DamageOverTime(amount, duration) => json!({
            "type": "DamageOverTime",
            "amount": amount,
            "duration": duration
        }),
        Effect::HealOverTime(hp, time) => json!({
            "type": "HealOverTime",
            "amount": hp,
            "duration": time
        }),
        Effect::RestoreResource(resource, amount) => json!({
            "type": "RestoreResource",
            "resource": format!("{:?}", resource), // Assuming Resource has Serialize
            "amount": amount
        }),
        Effect::Disarm => json!({ "type": "Disarm" }),
        Effect::PercentageDamage(percentage) => json!({
            "type": "PercentageDamage",
            "percentage": percentage
        }),
        Effect::ModifyStat(stat, amount, duration) => json!({
            "type": "ModifyStat",
            "stat": format!("{:?}", stat), // Assuming BaseStats has Serialize
            "amount": amount,
            "duration": duration
        }),
        Effect::Stun => json!({ "type": "Stun" }),
        Effect::Silence => json!({ "type": "Silence" }),
        // ... Similar for Root, Disarm ...
        Effect::Heal(amount) => json!({ "type": "Heal", "amount": amount }),
        // ... Similar for HealOverTime
        Effect::DrainResource(resource, amount) => json!({
            "type": "DrainResource",
            "resource": format!("{:?}", resource), // Assuming Resource has Serialize
            "amount": amount
        }),
        // ... Similar for RestoreResource
        Effect::CooldownReset(ability_id) => json!({
            "type": "CooldownReset",
            "ability_id": ability_id,
        }),
        Effect::TriggerEffect(effect) => effect_to_json(effect),
    }
}

pub fn effects_from_json(json_value: &Value) -> Result<Vec<Effect>, String> {
    let effects_array = json_value
        .as_array()
        .ok_or("Expected a JSON array of effects")?;

    let mut effects: Vec<Effect> = Vec::new();
    for effect_value in effects_array {
        let effect = effect_from_json(effect_value)?; // Use our existing single-effect deserialization
        effects.push(effect);
    }

    Ok(effects)
}

// JSON -> Effect
fn effect_from_json(json_value: &Value) -> Result<Effect, String> {
    let effect_type = json_value
        .get("type")
        .and_then(|t| t.as_str())
        .ok_or("Missing 'type' field in effect JSON")?;

    match effect_type {
        "Damage" => {
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for Damage effect")? as i32;
            Ok(Effect::Damage(amount))
        }
        "DamageOverTime" => {
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for DamageOverTime effect")?
                as i32;
            let duration = json_value
                .get("duration")
                .and_then(|d| d.as_i64())
                .ok_or("Invalid 'duration' for DamageOverTime effect")?
                as i32;
            Ok(Effect::DamageOverTime(amount, duration))
        }
        "PercentageDamage" => {
            let percentage = json_value
                .get("percentage")
                .and_then(|p| p.as_f64())
                .ok_or("Invalid 'percentage' for PercentageDamage effect")?
                as f32;
            Ok(Effect::PercentageDamage(percentage))
        }
        "ModifyStat" => {
            let stat: BaseStats = serde_json::from_value(json_value["stat"].clone())
                .map_err(|e| format!("Error deserializing 'stat': {}", e))?;
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for ModifyStat effect")? as i32;
            let duration = json_value
                .get("duration")
                .and_then(|d| d.as_i64())
                .ok_or("Invalid 'duration' for ModifyStat effect")?
                as i32;
            Ok(Effect::ModifyStat(stat, amount, duration))
        }
        "Stun" => Ok(Effect::Stun),
        "Silence" => Ok(Effect::Silence),
        "Disarm" => Ok(Effect::Disarm),
        "Heal" => {
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for Heal effect")? as i32;
            Ok(Effect::Heal(amount))
        }
        "HealOverTime" => {
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for HealOverTime effect")? as i32;
            let duration = json_value
                .get("duration")
                .and_then(|d| d.as_i64())
                .ok_or("Invalid 'duration' for HealOverTime effect")?
                as i32;
            Ok(Effect::HealOverTime(amount, duration))
        }
        "DrainResource" => {
            let resource: Resource = serde_json::from_value(json_value["resource"].clone())
                .map_err(|e| format!("Error deserializing 'resource': {}", e))?;
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for DrainResource effect")?
                as i32;
            Ok(Effect::DrainResource(resource, amount))
        }
        "RestoreResource" => {
            let resource: Resource = serde_json::from_value(json_value["resource"].clone())
                .map_err(|e| format!("Error deserializing 'resource': {}", e))?;
            let amount = json_value
                .get("amount")
                .and_then(|a| a.as_i64())
                .ok_or("Invalid 'amount' for RestoreResource effect")?
                as i32;
            Ok(Effect::RestoreResource(resource, amount))
        }
        "CooldownReset" => {
            let ability_id = json_value
                .get("ability_id")
                .and_then(|id| id.as_str())
                .ok_or("Invalid 'ability_id' for CooldownReset effect")?
                .to_string();
            Ok(Effect::CooldownReset(ability_id))
        }
        "TriggerEffect" => {
            let nested_effect = effect_from_json(
                json_value
                    .get("effect")
                    .ok_or("Missing 'effect' field for TriggerEffect")?,
            )?;
            Ok(Effect::TriggerEffect(Box::new(nested_effect)))
        }
        _ => Err(format!("Unsupported effect type: {}", effect_type)),
    }
}
