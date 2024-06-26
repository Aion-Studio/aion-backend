use serde::{Deserialize, Serialize};

use crate::prisma::{self, CardType, Class, EffectType, StatType, TargetType};

use crate::prisma::{card, deck};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Deck {
    pub id: String,
    pub name: String,
    pub hero_id: Option<String>,
    pub cards_in_deck: Vec<Card>,
    pub active: bool,
}

impl Deck {}

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

impl Default for Card {
    fn default() -> Self {
        Card {
            id: "".to_string(),
            name: "".to_string(),
            img_url: "".to_string(),
            cost: 0,
            class: Class::Fighter,
            last_attack_round: None,
            effects: Vec::new(),
            zeal: 0,
            tier: 0,
            card_type: CardType::Attack,
        }
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
#[serde(rename_all = "camelCase")]
pub struct CardEffect {
    pub id: String,
    pub card_id: String,
    pub effect: EffectType, // Updated to use the EffectType enum
    pub value: i32,
    pub target_type: TargetType,
    pub stat_affected: Option<StatType>,
    pub is_percentage_modifier: bool,
}

impl From<prisma::card_effect::Data> for CardEffect {
    fn from(data: prisma::card_effect::Data) -> Self {
        CardEffect {
            id: data.id,
            card_id: data.card_id,
            effect: data.effect_type,
            value: data.value,
            target_type: data.target,
            stat_affected: data.stat_affected,
            is_percentage_modifier: data.percentage_modifier,
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
