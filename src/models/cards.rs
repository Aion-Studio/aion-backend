use serde::{Deserialize, Serialize};
use tracing::info;

use crate::prisma::{self, hero_card};
use crate::prisma::{
    armor_effect, charge_effect, damage_effect, heal_effect, initiative_effect, lifesteal_effect,
    minion_effect, pickup_effect, poison_effect, resilience_effect, stun_effect, summon_effect,
    taunt_effect,
};
use crate::prisma::{card, deck, deck_card, spell_effect, DamageType, TargetType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deck {
    pub id: String,
    pub hero_id: Option<String>,
    pub cards_in_deck: Vec<Card>,
}

impl Deck {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub nation: Nation,
    pub rarity: Rarity,
    pub tier: i32,
    pub img_url: String,
    pub mana_cost: i32,
    pub health: i32,
    pub damage: i32,
    pub card_type: CardType,
    pub spell_effects: Vec<SpellEffect>,
    pub minion_effects: Vec<MinionEffect>,
    pub round_played: i32,
    pub last_attack_round: Option<i32>,
}

impl Card {
    pub fn attack(&self, target: &mut Card) {
        target.health -= self.damage;
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
            tier: 0,
            img_url: "".to_string(),
            mana_cost: 0,
            health: 0,
            damage: 0,
            card_type: CardType::Minion,
            spell_effects: Vec::new(),
            minion_effects: Vec::new(),
            round_played: 0,
            last_attack_round: None,
        }
    }
}

impl From<card::Data> for Card {
    fn from(data: card::Data) -> Self {
        Card {
            id: data.id,
            name: data.name,
            nation: data.nation.into(),
            rarity: data.rarity.into(),
            tier: data.tier,
            img_url: data.img_url,
            mana_cost: data.mana_cost,
            health: data.health,
            damage: data.damage,
            card_type: data.card_type.into(),
            round_played: 0,
            last_attack_round: None,
            spell_effects: data
                .spell_effects
                .map(|effects| effects.into_iter().map(SpellEffect::from).collect())
                .unwrap_or_default(),

            minion_effects: data
                .minion_effects
                .map(|effects| effects.into_iter().map(MinionEffect::from).collect())
                .unwrap_or_default(),
        }
    }
}

impl From<hero_card::Data> for Card {
    fn from(data: hero_card::Data) -> Self {
        //
        let card = data.card.unwrap();
        (*card).into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpellEffect {
    pub id: String,
    pub card_id: String,
    pub duration: i32,
    pub effect: SpellEffectType, // Updated to use the SpellEffectType enum
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MinionEffect {
    pub id: String,
    pub card_id: String,
    pub duration: i32,
    pub effect: MinionEffectType, // Updated to use the MinionEffectType enum
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MinionEffectType {
    Taunt(TauntEffect),
    Charge(ChargeEffect),
    Lifesteal(LifestealEffect),
    Pickup(PickupEffect),
    Summon(SummonEffect),
    Resilience(ResilienceEffect),
    Poison(PoisonEffect),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SpellEffectType {
    Damage(DamageEffect),
    Heal(HealEffect),
    Armor(ArmorEffect),
    Resilience(ResilienceEffect),
    Poison(PoisonEffect),
    Initiative(InitiativeEffect),
    Stun(StunEffect),
}

impl From<spell_effect::Data> for SpellEffect {
    fn from(data: spell_effect::Data) -> Self {
        let effect_type = data
            .effects
            .unwrap_or_default()
            .into_iter()
            .map(|effect_data| {
                effect_data
                    .damage_effect
                    .and_then(|inner| inner)
                    .map(|e| SpellEffectType::Damage(DamageEffect::from(*e)))
                    .or_else(|| {
                        effect_data
                            .heal_effect
                            .and_then(|inner| inner)
                            .map(|e| SpellEffectType::Heal(HealEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .armor_effect
                            .and_then(|inner| inner)
                            .map(|e| SpellEffectType::Armor(ArmorEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .resilience_effect
                            .and_then(|inner| inner)
                            .map(|e| SpellEffectType::Resilience(ResilienceEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .poison_effect
                            .and_then(|inner| inner)
                            .map(|e| SpellEffectType::Poison(PoisonEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .initiative_effect
                            .and_then(|inner| inner)
                            .map(|e| SpellEffectType::Initiative(InitiativeEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .stun_effect
                            .and_then(|inner| inner)
                            .map(|e| SpellEffectType::Stun(StunEffect::from(*e)))
                    })
                    .expect("Expected  one effect type to be present in the effects vector")
            })
            .next()
            .expect("Expected at least one effect type to be present in the effects vector");

        SpellEffect {
            id: data.id,
            card_id: data.card_id,
            duration: data.duration,
            effect: effect_type,
        }
    }
}
impl From<minion_effect::Data> for MinionEffect {
    fn from(data: minion_effect::Data) -> Self {
        let effect_type = data
            .effects
            .unwrap_or_default()
            .into_iter()
            .map(|effect_data| {
                effect_data
                    .taunt_effect
                    .and_then(|inner| inner)
                    .map(|e| MinionEffectType::Taunt(TauntEffect::from(*e)))
                    .or_else(|| {
                        effect_data
                            .charge_effect
                            .and_then(|inner| inner)
                            .map(|e| MinionEffectType::Charge(ChargeEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .lifesteal_effect
                            .and_then(|inner| inner)
                            .map(|e| MinionEffectType::Lifesteal(LifestealEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .pickup_effect
                            .and_then(|inner| inner)
                            .map(|e| MinionEffectType::Pickup(PickupEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .summon_effect
                            .and_then(|inner| inner)
                            .map(|e| MinionEffectType::Summon(SummonEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .resilience_effect
                            .and_then(|inner| inner)
                            .map(|e| MinionEffectType::Resilience(ResilienceEffect::from(*e)))
                    })
                    .or_else(|| {
                        effect_data
                            .poison_effect
                            .and_then(|inner| inner)
                            .map(|e| MinionEffectType::Poison(PoisonEffect::from(*e)))
                    })
                    // return empty array
                    .expect("inner failure onn the effects vector")
            })
            .next()
            .expect("Expected at least one effect type to be present in the effects vector");

        MinionEffect {
            id: data.id,
            card_id: data.card_id,
            duration: data.duration,
            effect: effect_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageEffect {
    pub id: String,
    // pub amount: i32,
    pub damage: Vec<(DamageType, TargetType, i32)>, // pub damage_type: DamageType,
                                                    // pub target_type: TargetType,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PickupEffect {
    pub id: String,
    pub amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SummonEffect {
    pub id: String,
}

impl From<damage_effect::Data> for DamageEffect {
    fn from(data: damage_effect::Data) -> Self {
        DamageEffect {
            id: data.id,
            damage: data
                .damage
                .unwrap()
                .into_iter()
                .map(|d| (d.damage_type.into(), d.target_type.into(), d.amount))
                .collect(),
        }
    }
}

impl From<heal_effect::Data> for HealEffect {
    fn from(data: heal_effect::Data) -> Self {
        HealEffect {
            id: data.id,
            amount: data.amount,
            target_type: data.target_type.into(),
        }
    }
}

impl From<armor_effect::Data> for ArmorEffect {
    fn from(data: armor_effect::Data) -> Self {
        ArmorEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}

impl From<resilience_effect::Data> for ResilienceEffect {
    fn from(data: resilience_effect::Data) -> Self {
        ResilienceEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}

impl From<poison_effect::Data> for PoisonEffect {
    fn from(data: poison_effect::Data) -> Self {
        PoisonEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}

impl From<initiative_effect::Data> for InitiativeEffect {
    fn from(data: initiative_effect::Data) -> Self {
        InitiativeEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}

impl From<stun_effect::Data> for StunEffect {
    fn from(data: stun_effect::Data) -> Self {
        StunEffect { id: data.id }
    }
}

impl From<taunt_effect::Data> for TauntEffect {
    fn from(data: taunt_effect::Data) -> Self {
        TauntEffect { id: data.id }
    }
}

impl From<charge_effect::Data> for ChargeEffect {
    fn from(data: charge_effect::Data) -> Self {
        ChargeEffect { id: data.id }
    }
}

impl From<lifesteal_effect::Data> for LifestealEffect {
    fn from(data: lifesteal_effect::Data) -> Self {
        LifestealEffect { id: data.id }
    }
}

impl From<pickup_effect::Data> for PickupEffect {
    fn from(data: pickup_effect::Data) -> Self {
        PickupEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}

impl From<summon_effect::Data> for SummonEffect {
    fn from(data: summon_effect::Data) -> Self {
        SummonEffect { id: data.id }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardType {
    Spell,
    Minion,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeckCard {
    deck_id: String,
    card: Card,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CardEffect {
    id: String,
    card_id: String,
    effect: EffectType,
    value: Option<i32>,
    duration: Option<i32>,
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
    Magic,
    Epic,
    Legendary,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EffectType {
    PhysicalDamage,
    SpellDamage,
    ChaosDamage,
    DamageOverTime,
    Stun,
    ReduceArmor,
    ReduceResilience,
    IncreaseArmor,
    IncreaseResilience,
    Heal,
    HealOverTime,
    DrawCards,
    ApplyPoison,
    RemovePoison,
    ApplyInitiative,
    RemoveInitiative,
}

impl From<deck::Data> for Deck {
    fn from(data: deck::Data) -> Self {
        // shouldnt be called anywhere right now
        Deck {
            id: data.id,
            hero_id: data.hero.unwrap().map(|hero| hero.id),
            cards_in_deck: Vec::new(), // Initialize an empty vector for now
        }
    }
}

impl From<deck_card::Data> for DeckCard {
    fn from(data: deck_card::Data) -> Self {
        DeckCard {
            deck_id: data.deck_id,
            card: (*data.card.unwrap()).into(),
            quantity: data.quantity,
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
            prisma::Rarity::Magic => Rarity::Magic,
            prisma::Rarity::Epic => Rarity::Epic,
            prisma::Rarity::Legendary => Rarity::Legendary,
        }
    }
}

impl From<Rarity> for prisma::Rarity {
    fn from(rarity: Rarity) -> Self {
        match rarity {
            Rarity::Common => prisma::Rarity::Common,
            Rarity::Magic => prisma::Rarity::Magic,
            Rarity::Epic => prisma::Rarity::Epic,
            Rarity::Legendary => prisma::Rarity::Legendary,
        }
    }
}
