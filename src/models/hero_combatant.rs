use std::any::Any;
use std::cmp::max;

use serde::{Deserialize, Serialize};

use crate::events::combat::CombatError;
use crate::models::cards::{Card, Deck};
use crate::models::combatant::Combatant;
use crate::models::hero::{Attributes, BaseStats, Inventory, Range};
use crate::models::talent::Talent;
use crate::prisma::DamageType;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeroCombatant {
    id: String,
    name: String,
    pub base_stats: BaseStats,
    pub attributes: Attributes,
    pub inventory: Inventory,
    pub deck: Deck,
    pub mana: i32,
    cards_in_discard: Vec<Card>,
    cards_in_hand: Vec<Card>,
    talents: Vec<Talent>,
}

impl HeroCombatant {
    pub fn new(
        id: String,
        name: String,
        base_stats: BaseStats,
        attributes: Attributes,
        inventory: Inventory,
        deck: Deck,
        mana: i32,
    ) -> Self {
        HeroCombatant {
            id,
            name,
            base_stats,
            attributes,
            inventory,
            deck,
            mana,
            cards_in_discard: vec![],
            cards_in_hand: vec![],
            talents: vec![],
        }
    }
}
impl Combatant for HeroCombatant {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_hp(&self) -> i32 {
        self.base_stats.hit_points
    }

    fn get_damage(&self) -> i32 {
        self.base_stats.damage.roll()
    }

    fn get_mana(&self) -> i32 {
        self.mana
    }
    fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }

    fn get_damage_stats(&self) -> Range<i32> {
        self.base_stats.damage.clone()
    }

    fn get_armor(&self) -> i32 {
        self.base_stats.armor
    }

    fn get_level(&self) -> i32 {
        self.base_stats.level
    }

    fn take_damage(&mut self, damage: i32, damage_type: DamageType) {
        match damage_type {
            DamageType::Physical => {
                let diff = damage - self.base_stats.armor;
                self.base_stats.armor = max(0, self.base_stats.armor - damage);
                if diff > 0 {
                    self.base_stats.hit_points -= diff;
                }
            }
            DamageType::Spell => {
                let diff = damage - self.base_stats.resilience;
                self.base_stats.resilience = max(0, self.base_stats.resilience - damage);
                if diff > 0 {
                    self.base_stats.hit_points -= diff;
                }
            }
            DamageType::Chaos => {
                self.base_stats.hit_points -= damage;
            }
        }
    }

    fn shuffle_deck(&mut self) {
        let deck = &mut self.deck;
        if self.cards_in_discard.len() > 0 {
            deck.cards_in_deck.append(&mut self.cards_in_discard);
            self.cards_in_discard.clear();
        }
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();

        deck.cards_in_deck.shuffle(&mut rng);
    }

    fn draw_cards(&mut self, num_cards: i32) {
        if self.deck.cards_in_deck.len() < num_cards as usize {
            self.shuffle_deck();
        }
        if self.deck.cards_in_deck.is_empty() {
            return;
        }
        self.cards_in_hand.append(
            &mut self
                .deck
                .cards_in_deck
                .drain(0..num_cards as usize)
                .collect::<Vec<Card>>(),
        );
    }

    fn add_to_discard(&mut self, card: Card) {
        self.cards_in_discard.push(card);
    }
    /// Sets the hero's mana to the amount
    fn add_mana(&mut self, mana: i32) {
        self.mana = mana;
    }

    fn spend_mana(&mut self, mana: i32) {
        self.mana -= mana;
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.cards_in_hand
    }
    fn get_resilience(&self) -> i32 {
        self.base_stats.resilience
    }

    fn play_card(&mut self, card: &Card) -> Result<(), CombatError> {
        if let Some(idx) = self.cards_in_hand.iter().position(|c| c.id == card.id) {
            self.cards_in_hand.remove(idx);
            // self.add_to_discard(card.clone());
            Ok(())
        } else {
            Err(CombatError::CardNotInHand)
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
