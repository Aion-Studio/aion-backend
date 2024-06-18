use serde::{Deserialize, Serialize};

use crate::prisma::{self, Class, EffectType, TargetType};

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
    pub name: String,
    pub img_url: String,
    pub cost: i32,
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
            last_attack_round: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CardEffect {
    pub id: String,
    pub card_id: String,
    pub effect: EffectType, // Updated to use the EffectType enum
    pub value: i32,
    pub target_type: TargetType,
}

impl Serialize for CardEffect {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // serialize self.effect and just return that
        self.effect.serialize(serializer)
    }
}

impl From<prisma::card_effect::Data> for CardEffect {
    fn from(data: prisma::card_effect::Data) -> Self {
        CardEffect {
            id: data.id,
            card_id: data.card_id,
            effect: data.efffect_type,
            value: data.value,
            target_type: data.target,
        }
    }
}

// --------------------------              SPELL EFFECTS                ---------------------------
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageEffect {
    pub id: String,
    // pub amount: i32,
    pub damage: Vec<i32>, // pub damage_type: DamageType,
                          // pub target_type: TargetType,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BattleCryEffect {
    pub id: String,
    pub amount: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CowardiceCurseEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PhantomTouchEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SprayOfKnivesEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HealEffect {
    pub id: String,
    pub amount: i32,
    pub target_type: TargetType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ArmorEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResilienceEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PoisonEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InitiativeEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StunEffect {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SummonEffect {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DazeEffect {
    pub id: String,
}
// --------------------------              MINION EFFECTS                ---------------------------

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TauntEffect {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChargeEffect {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LifestealEffect {
    pub id: String,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PickupEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EtherealEffect {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TwinEffect {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CleanseEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlockEffect {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoarAuraEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DyingWishHealEffect {
    pub id: String,
    pub amount: i32,
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
