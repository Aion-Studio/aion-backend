use serde::{Deserialize, Serialize};
use tracing::info;

use crate::prisma::{self, daze_effect};
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
    pub card_effects: Vec<CardEffect>, // Updated to use card_effects
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
            card_effects: Vec::new(),
            card_type: CardType::Minion,
            round_played: 0,
            last_attack_round: None,
        }
    }
}

impl From<(card::Data, Vec<CardEffect>)> for Card {
    fn from((data, card_effects): (card::Data, Vec<CardEffect>)) -> Self {
        Card {
            id: data.id,
            name: data.name,
            nation: data.nation.into(),
            rarity: data.rarity.into(),
            img_url: data.img_url,
            mana_cost: data.mana_cost,
            health: data.health,
            damage: data.damage,
            card_type: data.card_type.into(),
            card_effects,
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
        let effect = if let Some(Some(spell_effect)) = data.spell_effect {
            EffectType::SpellEffect(SpellEffectType::from(*spell_effect))
        } else if let Some(Some(minion_effect)) = data.minion_effect {
            EffectType::MinionEffect(MinionEffectType::from(*minion_effect))
        } else {
            EffectType::None
        };

        CardEffect {
            id: data.id,
            card_id: data.card_id,
            effect,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EffectType {
    SpellEffect(SpellEffectType),
    MinionEffect(MinionEffectType),
    None,
}

// --------------------------              MINION EFFECT TYPES                ---------------------------
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MinionEffectType {
    Taunt(TauntEffect),
    Charge(ChargeEffect),
    Lifesteal(LifestealEffect),
    Pickup(PickupEffect),
    // Summon(SummonEffect),  -- Summoning is a Spell Effect
    Ethereal(EtherealEffect),
    Twin(TwinEffect),
    Cleanse(CleanseEffect),
    Block(BlockEffect),
    RoarAura(RoarAuraEffect),
    DyingWishHeal(DyingWishHealEffect),
    None,
}
// --------------------------              SPELL EFFECT TYPES                ---------------------------
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SpellEffectType {
    Damage(DamageEffect),
    Heal(HealEffect),
    Armor(ArmorEffect),
    Resilience(ResilienceEffect),
    Poison(PoisonEffect),
    Initiative(InitiativeEffect),
    Stun(StunEffect),
    BattleCry(BattleCryEffect),
    CowardiceCurse(CowardiceCurseEffect),
    PhantomTouch(PhantomTouchEffect),
    SprayOfKnives(SprayOfKnivesEffect),
    Daze(DazeEffect),
    None,
}

impl From<prisma::spell_effect::Data> for SpellEffectType {
    fn from(data: prisma::spell_effect::Data) -> Self {
        // Map the inner specific effect data to the corresponding enum variant
        if let Some(Some(damage_effect)) = data.damage_effect {
            SpellEffectType::Damage(DamageEffect::from(*damage_effect))
        } else if let Some(Some(heal_effect)) = data.heal_effect {
            SpellEffectType::Heal(HealEffect::from(*heal_effect))
        } else if let Some(Some(armor_effect)) = data.armor_effect {
            SpellEffectType::Armor(ArmorEffect::from(*armor_effect))
        } else if let Some(Some(resilience_effect)) = data.resilience_effect {
            SpellEffectType::Resilience(ResilienceEffect::from(*resilience_effect))
        } else if let Some(Some(poison_effect)) = data.poison_effect {
            SpellEffectType::Poison(PoisonEffect::from(*poison_effect))
        } else if let Some(Some(initiative_effect)) = data.initiative_effect {
            SpellEffectType::Initiative(InitiativeEffect::from(*initiative_effect))
        } else if let Some(Some(stun_effect)) = data.stun_effect {
            SpellEffectType::Stun(StunEffect::from(*stun_effect))
        } else if let Some(Some(battle_cry_effect)) = data.battle_cry_effect {
            SpellEffectType::BattleCry(BattleCryEffect::from(*battle_cry_effect))
        } else if let Some(Some(cowardice_curse_effect)) = data.cowardice_curse_effect {
            SpellEffectType::CowardiceCurse(CowardiceCurseEffect::from(*cowardice_curse_effect))
        } else if let Some(Some(phantom_touch_effect)) = data.phantom_touch_effect {
            SpellEffectType::PhantomTouch(PhantomTouchEffect::from(*phantom_touch_effect))
        } else if let Some(Some(spray_of_knives_effect)) = data.spray_of_knives_effect {
            SpellEffectType::SprayOfKnives(SprayOfKnivesEffect::from(*spray_of_knives_effect))
        } else if let Some(Some(daze_effect)) = data.daze_effect {
            SpellEffectType::Daze(DazeEffect::from(*daze_effect))
        } else {
            SpellEffectType::None
        }
    }
}

impl From<prisma::minion_effect::Data> for MinionEffectType {
    fn from(data: prisma::minion_effect::Data) -> Self {
        // Map the inner specific effect data to the corresponding enum variant
        if let Some(Some(taunt_effect)) = data.taunt_effect {
            MinionEffectType::Taunt(TauntEffect::from(*taunt_effect))
        } else if let Some(Some(charge_effect)) = data.charge_effect {
            MinionEffectType::Charge(ChargeEffect::from(*charge_effect))
        } else if let Some(Some(lifesteal_effect)) = data.lifesteal_effect {
            MinionEffectType::Lifesteal(LifestealEffect::from(*lifesteal_effect))
        } else if let Some(Some(pickup_effect)) = data.pickup_effect {
            MinionEffectType::Pickup(PickupEffect::from(*pickup_effect))
        } else if let Some(Some(ethereal_effect)) = data.ethereal_effect {
            MinionEffectType::Ethereal(EtherealEffect::from(*ethereal_effect))
        } else if let Some(Some(twin_effect)) = data.twin_effect {
            MinionEffectType::Twin(TwinEffect::from(*twin_effect))
        } else if let Some(Some(cleanse_effect)) = data.cleanse_effect {
            MinionEffectType::Cleanse(CleanseEffect::from(*cleanse_effect))
        } else if let Some(Some(block_effect)) = data.block_effect {
            MinionEffectType::Block(BlockEffect::from(*block_effect))
        } else if let Some(Some(roar_aura_effect)) = data.roar_aura_effect {
            MinionEffectType::RoarAura(RoarAuraEffect::from(*roar_aura_effect))
        } else if let Some(Some(dying_wish_heal_effect)) = data.dying_wish_heal_effect {
            MinionEffectType::DyingWishHeal(DyingWishHealEffect::from(*dying_wish_heal_effect))
        } else {
            MinionEffectType::None
        }
    }
}

// --------------------------              SPELL EFFECTS                ---------------------------
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageEffect {
    pub id: String,
    // pub amount: i32,
    pub damage: Vec<(DamageType, TargetType, i32)>, // pub damage_type: DamageType,
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

// --------------------------              FROM IMPLS                ---------------------------

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

impl From<daze_effect::Data> for DazeEffect {
    fn from(data: daze_effect::Data) -> Self {
        DazeEffect { id: data.id }
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
impl From<battle_cry_effect::Data> for BattleCryEffect {
    fn from(data: battle_cry_effect::Data) -> Self {
        BattleCryEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}
impl From<cowardice_curse_effect::Data> for CowardiceCurseEffect {
    fn from(data: cowardice_curse_effect::Data) -> Self {
        CowardiceCurseEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}
impl From<phantom_touch_effect::Data> for PhantomTouchEffect {
    fn from(data: phantom_touch_effect::Data) -> Self {
        PhantomTouchEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}
impl From<spray_of_knives_effect::Data> for SprayOfKnivesEffect {
    fn from(data: spray_of_knives_effect::Data) -> Self {
        SprayOfKnivesEffect {
            id: data.id,
            amount: data.amount,
        }
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
        LifestealEffect {
            id: data.id,
            percentage: data.percentage,
        }
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

impl From<ethereal_effect::Data> for EtherealEffect {
    fn from(data: ethereal_effect::Data) -> Self {
        EtherealEffect { id: data.id }
    }
}
impl From<twin_effect::Data> for TwinEffect {
    fn from(data: twin_effect::Data) -> Self {
        TwinEffect { id: data.id }
    }
}
impl From<cleanse_effect::Data> for CleanseEffect {
    fn from(data: cleanse_effect::Data) -> Self {
        CleanseEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}
impl From<block_effect::Data> for BlockEffect {
    fn from(data: block_effect::Data) -> Self {
        BlockEffect { id: data.id }
    }
}
impl From<roar_aura_effect::Data> for RoarAuraEffect {
    fn from(data: roar_aura_effect::Data) -> Self {
        RoarAuraEffect {
            id: data.id,
            amount: data.amount,
        }
    }
}
impl From<dying_wish_heal_effect::Data> for DyingWishHealEffect {
    fn from(data: dying_wish_heal_effect::Data) -> Self {
        DyingWishHealEffect {
            id: data.id,
            amount: data.amount,
        }
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
