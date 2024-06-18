use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::prisma::{hero_spell, EffectType, TargetType};

use super::resources::Resource;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub duration: i32,
    pub effects: Vec<SpellEffect>,
}

impl From<hero_spell::Data> for Spell {
    fn from(data: hero_spell::Data) -> Self {
        let spell_unwrapped = match data.spell.flatten() {
            Some(spell) => *spell,
            None => panic!("Spell data is missing"),
        };
        let effects: Vec<SpellEffect> = spell_unwrapped.effects().map_or(vec![], |effects| {
            effects
                .iter()
                .map(|effect| SpellEffect {
                    id: effect.id.clone(),
                    value: effect.value.clone(),
                    target: effect.target.clone(),
                    effect: effect.effect.clone(),
                })
                .collect()
        });

        Self {
            id: spell_unwrapped.id,
            name: spell_unwrapped.name.clone(),
            level: spell_unwrapped.level,
            duration: spell_unwrapped.duration.unwrap(),
            effects,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellEffect {
    pub id: String,
    pub value: i32,
    pub target: TargetType,
    pub effect: EffectType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Talent {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cooldown: i32, // Turns remaining until usable again
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Effect {
    // Damage Effects
    Damage(i32),              // Amount of damage to inflict immediately
    DamageOverTime(i32, i32), // Damage to deal per turn, number of turns to apply the damage
    PercentageDamage(f32),    // Percentage of the target's maximum health to inflict as damage

    // Stat Manipulation Effects

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
            todo!()
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
