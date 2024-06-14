use serde::{Deserialize, Serialize};
use tracing::info;

use crate::prisma::{self, daze_effect, EffectType};
use crate::prisma::{
    armor_effect, battle_cry_effect, block_effect, charge_effect, cleanse_effect,
    cowardice_curse_effect, damage_effect, dying_wish_heal_effect, ethereal_effect, heal_effect,
    initiative_effect, lifesteal_effect, phantom_touch_effect, pickup_effect, poison_effect,
    resilience_effect, roar_aura_effect, spray_of_knives_effect, stun_effect, taunt_effect,
    twin_effect, DamageType, TargetType,
};
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
    pub name: String,
    pub nation: Nation,
    pub rarity: Rarity,
    pub img_url: String,
    pub mana_cost: i32,
    pub health: i32,
    pub damage: i32,
    pub card_type: CardType,
    pub effects: Vec<CardEffect>, // Updated to use card_effects
    pub round_played: i32,
    pub last_attack_round: Option<i32>,
}

impl Card {
    pub fn attack(&self, target: &mut Card) {
        target.health -= self.damage;
        info!(
            "{:?} attacks {:?} for {:?} damage",
            self.name, target.name, self.damage
        );
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            id: "".to_string(),
            name: "".to_string(),
            nation: Nation::Dusane,
            rarity: Rarity::Common,
            img_url: "".to_string(),
            mana_cost: 0,
            health: 0,
            damage: 0,
            round_played: 0,
            last_attack_round: None,
        }
    }
}

impl From<(card::Data, Vec<CardEffect>)> for Card {
    fn from((data, effects): (card::Data, Vec<CardEffect>)) -> Self {
        Card {
            id: data.id,
            name: data.name,
            cost: data.cost,
            effects,
            round_played: 0,
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
    pub damage: Vec<(i32)>, // pub damage_type: DamageType,
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
impl From<prisma::CardType> for CardType {
    fn from(card_type: prisma::CardType) -> Self {
        match card_type {
            prisma::CardType::Spell => CardType::Spell,
            prisma::CardType::Minion => CardType::Minion,
        }
    }
}

impl From<CardType> for prisma::CardType {
    fn from(card_type: CardType) -> Self {
        match card_type {
            CardType::Spell => prisma::CardType::Spell,
            CardType::Minion => prisma::CardType::Minion,
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
