use serde::{Deserialize, Serialize};

use crate::prisma::{hero_spell, spell, spell_effect, DamageType, EffectType, TargetType};

use super::cards::Effect;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub duration: i32,
    pub effects: Vec<SpellEffect>,
}

impl Spell {
    pub fn fireball(damage: i32) -> Self {
        let effect = SpellEffect {
            target_type: TargetType::Opponent,
            duration: None,
            effect: Effect::Damage {
                value: damage,
                damage_type: DamageType::Chaos,
            },
        };
        let spell = Self {
            id: "1".to_string(),
            name: "Fireball".to_string(),
            level: 1,
            duration: 1,
            effects: vec![effect],
        };

        spell
    }

    pub fn bolt(damage: i32) -> Self {
        let effect = SpellEffect {
            target_type: TargetType::Opponent,
            duration: None,
            effect: Effect::Damage {
                value: damage,
                damage_type: DamageType::Normal,
            },
        };
        let spell = Self {
            id: "2".to_string(),
            name: "Bolt".to_string(),
            level: 1,
            duration: 1,
            effects: vec![effect],
        };

        spell
    }

    pub fn initiative(amount: i32) -> Self {
        let effect = SpellEffect {
            target_type: TargetType::Opponent,
            duration: None,
            effect: Effect::BuffInitiative { value: amount },
        };
        let spell = Self {
            id: "3".to_string(),
            name: "Initiative".to_string(),
            level: 1,
            duration: 1,
            effects: vec![effect],
        };

        spell
    }

    pub fn buff_armor(amount: i32) -> Self {
        let effect = SpellEffect {
            target_type: TargetType::Itself,
            duration: None,
            effect: Effect::BuffArmor { value: amount },
        };
        let spell = Self {
            id: "4".to_string(),
            name: "Buff Armor".to_string(),
            level: 1,
            duration: 1,
            effects: vec![effect],
        };

        spell
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum CombatModifier {
    Initiative(i32),
    Damage(i32),
    Resillience(i32),
    Poision(i32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellEffect {
    pub target_type: TargetType,
    pub duration: Option<i32>,
    pub effect: Effect,
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
                .map(|effect| (*effect).clone().into())
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

impl From<spell_effect::Data> for SpellEffect {
    fn from(data: spell_effect::Data) -> Self {
        match data.effect_type {
            EffectType::Damage => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::Damage {
                    value: data.value,
                    damage_type: data.damage_type.unwrap_or(DamageType::Normal),
                },
            },
            EffectType::Heal => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::Heal { value: data.value },
            },

            EffectType::Silence => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::Silence,
            },

            EffectType::BuffDamage => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::BuffDamage { value: data.value },
            },
            EffectType::DebuffDamage => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::DebuffDamage { value: data.value },
            },
            EffectType::ManaGain => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::ManaGain { value: data.value },
            },
            EffectType::Draw => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::Draw { value: data.value },
            },

            EffectType::Poison => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::Poison { value: data.value },
            },
            EffectType::BuffInitiative => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::BuffInitiative { value: data.value },
            },
            EffectType::DebuffInitiative => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::DebuffInitiative { value: data.value },
            },
            EffectType::BuffArmor => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::BuffArmor { value: data.value },
            },
            EffectType::DebuffArmor => Self {
                target_type: data.target,
                duration: data.duration,
                effect: Effect::DebuffArmor { value: data.value },
            },
        }
    }
}
