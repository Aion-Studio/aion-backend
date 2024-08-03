use serde::{Deserialize, Serialize};

use crate::prisma::{self, CardType, Class, DamageType, EffectType, StatType, TargetType};
use crate::prisma::{card, deck};

use super::talent::CombatModifier;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    pub id: String,
    pub name: String,
    pub hero_id: Option<String>,
    pub cards_in_deck: Vec<Card>,
    pub active: bool,
}

impl Default for Deck {
    fn default() -> Self {
        // create a deck of 12 cards by calling Card::default, 12 item vector
        let cards_in_deck: Vec<Card> = (0..12).map(|_| Card::default()).collect();
        let rand_name = [
            "Deck of the Gods",
            "Deck of the Titans",
            "Deck of the Ancients",
        ];

        Deck {
            id: uuid::Uuid::new_v4().to_string(),
            name: rand_name[rand::random::<usize>() % rand_name.len()].to_string(),
            hero_id: None,
            cards_in_deck,
            active: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HeroCard {
    pub id: String,
    pub card: Card,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: String,
    pub class: Class,
    pub card_type: CardType,
    pub name: String,
    pub img_url: String,
    pub cost: i32,
    pub zeal: i32,
    pub tier: i32,
    pub effects: Vec<CardEffect>, // Updated to use card_effects
    pub last_attack_round: Option<i32>,
}

impl Card {
    pub fn poison(amount: i32, rounds: Option<i32>) -> Self {
        Card {
            id: uuid::Uuid::new_v4().to_string(),
            class: Class::Fighter,
            card_type: CardType::Attack,
            name: "Poison".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: "".to_string(),
                effect: Effect::Poison { value: amount },
                target_type: TargetType::Opponent,
                stat_affected: None,
                duration: rounds,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn initiative(amount: i32) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Card {
            id: id.clone(),
            class: Class::Fighter,
            card_type: CardType::Utility,
            name: "Initiative".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: id,
                effect: Effect::BuffInitiative { value: amount },
                target_type: TargetType::Opponent,
                stat_affected: None,
                duration: None,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn initiative_remove(amount: i32) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Card {
            id: id.clone(),
            class: Class::Fighter,
            card_type: CardType::Utility,
            name: "Initiative Remove".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: id,
                effect: Effect::DebuffInitiative { value: amount },
                target_type: TargetType::Itself,
                stat_affected: None,
                duration: None,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn buff_armor(amount: i32) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Card {
            id: id.clone(),
            class: Class::Fighter,
            card_type: CardType::Defensive,
            name: "Buff Armor".to_string(),
            img_url: "".to_string(),
            cost: 2,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: id,
                effect: Effect::BuffArmor { value: amount },
                target_type: TargetType::Itself,
                stat_affected: None,
                duration: None,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn debuff_armor(amount: i32) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Card {
            id: id.clone(),
            class: Class::Fighter,
            card_type: CardType::Defensive,
            name: "Debuff Armor".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: id,
                effect: Effect::DebuffArmor { value: amount },
                target_type: TargetType::Opponent,
                stat_affected: None,
                duration: None,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn buff_damage(amount: i32, rounds: Option<i32>) -> Self {
        Card {
            id: uuid::Uuid::new_v4().to_string(),
            class: Class::Fighter,
            card_type: CardType::Attack,
            name: "Buff Damage".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: "".to_string(),
                effect: Effect::BuffDamage { value: amount },
                target_type: TargetType::Itself,
                stat_affected: None,
                duration: rounds,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn debuff_damage(amount: i32) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Card {
            id: id.clone(),
            class: Class::Fighter,
            card_type: CardType::Attack,
            name: "Debuff Damage".to_string(),
            img_url: "".to_string(),
            cost: 2,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: id,
                effect: Effect::DebuffDamage { value: amount },
                target_type: TargetType::Opponent,
                stat_affected: None,
                duration: None,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn silence(rounds: i32) -> Self {
        Card {
            id: uuid::Uuid::new_v4().to_string(),
            class: Class::Fighter,
            card_type: CardType::Utility,
            name: "Silence".to_string(),
            img_url: "".to_string(),
            cost: 2,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: "".to_string(),
                effect: Effect::Silence,
                target_type: TargetType::Opponent,
                stat_affected: None,
                duration: Some(rounds),
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }

    pub fn attack(new_value: i32, damage_type: DamageType) -> Self {
        let card_type = CardType::Attack;
        let mut effect = CardEffect::get_random_by_card_type(card_type);
        match effect.effect {
            Effect::Damage {
                ref mut damage_type,
                ref mut value,
            } => {
                *damage_type = damage_type.clone();
                *value = new_value;
            }
            _ => {}
        }
        let card = Card {
            id: uuid::Uuid::new_v4().to_string(),
            class: Class::get_random(),
            card_type,
            name: "".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![effect],
            last_attack_round: None,
        };
        card
    }

    pub fn mana_gain_and_attack(amount_mana: i32, amount_attack: i32) -> Self {
        let card_id = uuid::Uuid::new_v4().to_string();
        let attack_effect = CardEffect {
            id: uuid::Uuid::new_v4().to_string(),
            card_id: card_id.clone(),
            effect: Effect::Damage {
                value: amount_attack,
                damage_type: DamageType::Normal,
            },
            target_type: TargetType::Opponent,
            stat_affected: None,
            duration: None,
            is_percentage_modifier: false,
        };
        Card {
            id: card_id.clone(),
            class: Class::Fighter,
            card_type: CardType::Utility,
            name: "Mana Gain".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![
                attack_effect,
                CardEffect {
                    id: uuid::Uuid::new_v4().to_string(),
                    card_id: card_id,
                    effect: Effect::ManaGain { value: amount_mana },
                    target_type: TargetType::Itself,
                    stat_affected: None,
                    duration: None,
                    is_percentage_modifier: false,
                },
            ],
            last_attack_round: None,
        }
    }

    pub fn heal(amount: i32) -> Self {
        Card {
            id: uuid::Uuid::new_v4().to_string(),
            class: Class::Fighter,
            card_type: CardType::Defensive,
            name: "Heal".to_string(),
            img_url: "".to_string(),
            cost: 2,
            zeal: 0,
            tier: 1,
            effects: vec![CardEffect {
                id: uuid::Uuid::new_v4().to_string(),
                card_id: "".to_string(),
                effect: Effect::Heal { value: amount },
                target_type: TargetType::Itself,
                stat_affected: None,
                duration: None,
                is_percentage_modifier: false,
            }],
            last_attack_round: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Effect {
    Damage { value: i32, damage_type: DamageType },
    Heal { value: i32 },
    BuffDamage { value: i32 },
    DebuffDamage { value: i32 },
    ManaGain { value: i32 },
    Draw { value: i32 },
    BuffArmor { value: i32 },
    DebuffArmor { value: i32 },
    Silence,
    Poison { value: i32 },
    BuffInitiative { value: i32 },
    DebuffInitiative { value: i32 },
    // Modifier(CombatModifier),
}

impl From<prisma::card_effect::Data> for CardEffect {
    fn from(data: prisma::card_effect::Data) -> Self {
        let effect = Effect::from(data.clone());
        CardEffect {
            id: data.id,
            card_id: data.card_id,
            effect,
            target_type: data.target,
            duration: data.duration,
            stat_affected: data.stat_affected,
            is_percentage_modifier: data.percentage_modifier,
        }
    }
}

impl From<prisma::card_effect::Data> for Effect {
    fn from(data: prisma::card_effect::Data) -> Self {
        match data.effect_type {
            prisma::EffectType::Damage => Effect::Damage {
                value: data.value,
                damage_type: data.damage_type.unwrap_or(DamageType::Normal),
            },
            prisma::EffectType::Heal => Effect::Heal { value: data.value },

            prisma::EffectType::BuffDamage => Effect::BuffDamage { value: data.value },

            prisma::EffectType::DebuffDamage => Effect::DebuffDamage { value: data.value },
            prisma::EffectType::ManaGain => Effect::ManaGain { value: data.value },
            prisma::EffectType::Draw => Effect::Draw { value: data.value },
            prisma::EffectType::Silence => Effect::Silence,
            prisma::EffectType::Poison => Effect::Poison { value: data.value },
            prisma::EffectType::BuffInitiative => Effect::BuffInitiative { value: data.value },
            prisma::EffectType::DebuffInitiative => Effect::DebuffInitiative { value: data.value },
            prisma::EffectType::BuffArmor => Effect::BuffArmor { value: data.value },
            prisma::EffectType::DebuffArmor => Effect::DebuffArmor { value: data.value },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CardEffect {
    pub id: String,
    pub card_id: String,
    pub effect: Effect, // Updated to use the EffectType enum
    pub target_type: TargetType,
    pub stat_affected: Option<StatType>,
    pub duration: Option<i32>,
    pub is_percentage_modifier: bool,
}

impl CardEffect {
    pub fn get_random_by_card_type(card_type: CardType) -> Self {
        match card_type {
            CardType::Attack => {
                //random damage between 1-e
                let damage = rand::random::<i32>() % 8;
                let effect = CardEffect {
                    id: uuid::Uuid::new_v4().to_string(),
                    card_id: "".to_string(),
                    effect: Effect::Damage {
                        value: damage,
                        damage_type: DamageType::Normal,
                    },
                    target_type: TargetType::Opponent,
                    stat_affected: None,
                    duration: None,
                    is_percentage_modifier: false,
                };
                effect
            }
            CardType::Defensive => {
                //random armor between 1-8
                let armor = rand::random::<i32>() % 8;
                CardEffect {
                    id: uuid::Uuid::new_v4().to_string(),
                    card_id: uuid::Uuid::new_v4().to_string(),
                    effect: Effect::BuffArmor { value: armor },
                    target_type: TargetType::Itself,
                    stat_affected: None,
                    duration: None,
                    is_percentage_modifier: false,
                }
            }
            CardType::Utility => {
                //random mana gain between 1-2
                let mana_gain = rand::random::<i32>() % 2;
                let effect_types = [
                    EffectType::ManaGain,
                    EffectType::Draw,
                    EffectType::Silence,
                    EffectType::Poison,
                    EffectType::BuffInitiative,
                ];
                let effect_type = effect_types[rand::random::<usize>() % effect_types.len()];
                let (target, value, duration) = match effect_type {
                    EffectType::ManaGain => {
                        (TargetType::Itself, rand::random::<i32>() % 2, Some(1))
                    }
                    EffectType::Draw => (TargetType::Itself, rand::random::<i32>() % 2, Some(1)),
                    EffectType::Silence => (TargetType::Opponent, 1, Some(1)),
                    EffectType::Poison => (TargetType::Opponent, 1, Some(2)),
                    EffectType::BuffInitiative => (TargetType::Itself, 1, Some(1)),
                    _ => (TargetType::Itself, 1, Some(1)),
                };

                CardEffect {
                    id: uuid::Uuid::new_v4().to_string(),
                    card_id: uuid::Uuid::new_v4().to_string(),
                    effect: match effect_type {
                        EffectType::ManaGain => Effect::ManaGain { value },
                        EffectType::Draw => Effect::Draw { value },
                        EffectType::Silence => Effect::Silence,
                        EffectType::Poison => Effect::Poison { value },
                        EffectType::BuffInitiative => Effect::BuffInitiative { value },
                        _ => Effect::ManaGain { value },
                    },
                    target_type: target,
                    stat_affected: None,
                    duration,
                    is_percentage_modifier: false,
                }
            }
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        //implement a card where cost is between 1-2 , zeal is between 0-10, tier is between 1-3,
        //card_type is a random selection of the enum variants, class is a random selection of the
        //enum variants and effects is also created with CardEffect but with random values
        let card_type = CardType::get_random();
        let effect = CardEffect::get_random_by_card_type(card_type);
        let card = Card {
            id: uuid::Uuid::new_v4().to_string(),
            class: Class::get_random(),
            card_type,
            name: "".to_string(),
            img_url: "".to_string(),
            cost: 1,
            zeal: 0,
            tier: 1,
            effects: vec![effect],
            last_attack_round: None,
        };
        card
    }
}

impl Class {
    pub fn get_random() -> Self {
        use Class::*;
        let classes = [Fighter, Ranger, Wizard];
        let index = rand::random::<usize>() % classes.len();
        classes[index]
    }
}

impl CardType {
    pub fn get_random() -> Self {
        use CardType::*;
        let card_types = [Attack, Defensive, Utility];
        let index = rand::random::<usize>() % card_types.len();
        card_types[index]
    }
}

impl From<(card::Data, Vec<CardEffect>)> for Card {
    fn from((data, effects): (card::Data, Vec<CardEffect>)) -> Self {
        Card {
            id: data.id,
            name: data.name,
            class: data.class,
            cost: data.cost,
            effects,
            img_url: data.img_url,
            zeal: data.zeal,
            tier: data.tier,
            card_type: data.card_type,
            last_attack_round: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeckCard {
    deck_id: String,
    card: Card,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Nation {
    Dusane,
    Aylen,
    Ironmark,
    Kelidor,
    Meta,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl From<deck::Data> for Deck {
    fn from(data: deck::Data) -> Self {
        Deck {
            id: data.id,
            name: data.name,
            hero_id: data.hero.unwrap().map(|hero| hero.id),
            cards_in_deck: Vec::new(), // Initialize an empty vector for now
            active: data.active,
        }
    }
}

impl From<prisma::Nation> for Nation {
    fn from(nation: prisma::Nation) -> Self {
        match nation {
            prisma::Nation::Dusane => Nation::Dusane,
            prisma::Nation::Aylen => Nation::Aylen,
            prisma::Nation::Ironmark => Nation::Ironmark,
            prisma::Nation::Kelidor => Nation::Kelidor,
            prisma::Nation::Meta => Nation::Meta,
        }
    }
}

impl From<Nation> for prisma::Nation {
    fn from(nation: Nation) -> Self {
        match nation {
            Nation::Dusane => prisma::Nation::Dusane,
            Nation::Aylen => prisma::Nation::Aylen,
            Nation::Ironmark => prisma::Nation::Ironmark,
            Nation::Kelidor => prisma::Nation::Kelidor,
            Nation::Meta => prisma::Nation::Meta,
        }
    }
}

impl From<prisma::Rarity> for Rarity {
    fn from(rarity: prisma::Rarity) -> Self {
        match rarity {
            prisma::Rarity::Common => Rarity::Common,
            prisma::Rarity::Rare => Rarity::Rare,
            prisma::Rarity::Epic => Rarity::Epic,
            prisma::Rarity::Legendary => Rarity::Legendary,
        }
    }
}

impl From<Rarity> for prisma::Rarity {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::Common => prisma::Rarity::Common,
            Rarity::Rare => prisma::Rarity::Rare,
            Rarity::Epic => prisma::Rarity::Epic,
            Rarity::Legendary => prisma::Rarity::Legendary,
        }
    }
}
