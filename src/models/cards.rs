use serde::{Deserialize, Serialize};

use crate::prisma;
use crate::prisma::{
    armor_effect, charge_effect, damage_effect, heal_effect, initiative_effect, lifesteal_effect,
    pickup_effect, poison_effect, resilience_effect, stun_effect, summon_effect, taunt_effect,
};
use crate::prisma::{card, DamageType, deck, deck_card, minion_effect, spell_effect, TargetType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deck {
    id: String,
    hero_id: Option<String>,
    cards_in_deck: Vec<Card>,
}

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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpellEffect {
    pub id: String,
    pub card_id: String,
    pub duration: i32,
    pub damage_effect: Option<DamageEffect>,
    pub heal_effect: Option<HealEffect>,
    pub armor_effect: Option<ArmorEffect>,
    pub resilience_effect: Option<ResilienceEffect>,
    pub poison_effect: Option<PoisonEffect>,
    pub initiative_effect: Option<InitiativeEffect>,
    pub stun_effect: Option<StunEffect>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MinionEffect {
    pub id: String,
    pub card_id: String,
    pub duration: i32,
    pub taunt_effect: Option<TauntEffect>,
    pub charge_effect: Option<ChargeEffect>,
    pub lifesteal_effect: Option<LifestealEffect>,
    pub pickup_effect: Option<PickupEffect>,
    pub summon_effect: Option<SummonEffect>,
    pub resilience_effect: Option<ResilienceEffect>,
}

impl From<spell_effect::Data> for SpellEffect {
    fn from(data: spell_effect::Data) -> Self {
        SpellEffect {
            id: data.id,
            card_id: data.card_id,
            duration: data.duration,
            damage_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .damage_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_damage_effect| {
                            DamageEffect::from((**boxed_damage_effect).clone())
                        })
                })
            }),

            heal_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .heal_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_heal_effect| HealEffect::from((**boxed_heal_effect).clone()))
                })
            }),
            armor_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .armor_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_armor_effect| ArmorEffect::from((**boxed_armor_effect).clone()))
                })
            }),
            resilience_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .resilience_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_resilience_effect| {
                            ResilienceEffect::from((**boxed_resilience_effect).clone())
                        })
                })
            }),
            poison_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .poison_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_poison_effect| {
                            PoisonEffect::from((**boxed_poison_effect).clone())
                        })
                })
            }),
            initiative_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .initiative_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_initiative_effect| {
                            InitiativeEffect::from((**boxed_initiative_effect).clone())
                        })
                })
            }),
            stun_effect: data.effects.as_ref().and_then(|effects| {
                effects.iter().find_map(|effect| {
                    effect
                        .stun_effect
                        .as_ref()?
                        .as_ref()
                        .map(|boxed_stun_effect| StunEffect::from((**boxed_stun_effect).clone()))
                })
            }),
        }
    }
}

impl From<minion_effect::Data> for MinionEffect {
    fn from(data: minion_effect::Data) -> Self {
        MinionEffect {
            id: data.id,
            card_id: data.card_id,
            duration: data.duration,
            taunt_effect: data.effects.as_ref().and_then(|effects| {
                effects
                    .iter()
                    .find_map(|effect| effect.taunt_effect.as_ref()?.as_ref())
                    .map(|boxed_taunt_effect| TauntEffect::from((**boxed_taunt_effect).clone()))
            }),
            charge_effect: data.effects.as_ref().and_then(|effects| {
                effects
                    .into_iter()
                    .find_map(|effect| effect.charge_effect.as_ref()?.as_ref())
                    .map(|boxed_charge_effect| ChargeEffect::from((**boxed_charge_effect).clone()))
            }),
            lifesteal_effect: data.effects.as_ref().and_then(|effects| {
                effects
                    .into_iter()
                    .find_map(|effect| effect.lifesteal_effect.as_ref()?.as_ref())
                    .map(|boxed_lifesteal_effect| {
                        LifestealEffect::from((**boxed_lifesteal_effect).clone())
                    })
            }),
            pickup_effect: data.effects.as_ref().and_then(|effects| {
                effects
                    .into_iter()
                    .find_map(|effect| effect.pickup_effect.as_ref()?.as_ref())
                    .map(|boxed_pickup_effect| PickupEffect::from((**boxed_pickup_effect).clone()))
            }),
            summon_effect: data.effects.as_ref().and_then(|effects| {
                effects
                    .into_iter()
                    .find_map(|effect| effect.summon_effect.as_ref()?.as_ref())
                    .map(|boxed_summon_effect| SummonEffect::from((**boxed_summon_effect).clone()))
            }),
            resilience_effect: data.effects.as_ref().and_then(|effects| {
                effects
                    .into_iter()
                    .find_map(|effect| effect.resilience_effect.as_ref()?.as_ref())
                    .map(|boxed_resilience_effect| {
                        ResilienceEffect::from((**boxed_resilience_effect).clone())
                    })
            }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageEffect {
    pub id: String,
    pub amount: i32,
    pub damage_type: DamageType,
    pub target_type: TargetType,
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
            amount: data.amount,
            damage_type: data.damage_type.into(),
            target_type: data.target_type.into(),
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

// impl From<effect_type::Data> for EffectType {
//     fn from(effect_type: effect_type::Data) -> Self {
//         match effect_type.name.as_str() {
//             "PhysicalDamage" => EffectType::PhysicalDamage,
//             "SpellDamage" => EffectType::SpellDamage,
//             "ChaosDamage" => EffectType::ChaosDamage,
//             "DamageOverTime" => EffectType::DamageOverTime,
//             "Stun" => EffectType::Stun,
//             "ReduceArmor" => EffectType::ReduceArmor,
//             "ReduceResilience" => EffectType::ReduceResilience,
//             "IncreaseArmor" => EffectType::IncreaseArmor,
//             "IncreaseResilience" => EffectType::IncreaseResilience,
//             "Heal" => EffectType::Heal,
//             "HealOverTime" => EffectType::HealOverTime,
//             "DrawCards" => EffectType::DrawCards,
//             "ApplyPoison" => EffectType::ApplyPoison,
//             "RemovePoison" => EffectType::RemovePoison,
//             "ApplyInitiative" => EffectType::ApplyInitiative,
//             "RemoveInitiative" => EffectType::RemoveInitiative,
//             _ => panic!("Effect type not found"),
//         }
//     }
// }
// fn convert_deck_data(deck_data: deck::Data) -> Deck {
//     Deck::from(deck_data)
// }
//
// fn convert_card_data(card_data: card::Data) -> Card {
//     Card::from(card_data)
// }
//
// fn convert_card_effect_data(card_effect_data: card_effect::Data) -> CardEffect {
//     CardEffect::from(card_effect_data)
// }
