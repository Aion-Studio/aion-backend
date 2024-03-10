use std::any::Any;

use serde::{Deserialize, Serialize};

use crate::models::cards::{Card, Deck};
use crate::models::combatant::Combatant;
use crate::models::hero::{Attributes, BaseStats, Inventory, Range};
use crate::models::talent::Talent;

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
    fn attack(&self, other: &mut dyn Combatant) {
        let damage = self.get_damage();
        other.take_damage(damage);
    }
    fn take_damage(&mut self, damage: i32) {
        self.base_stats.hit_points -= damage;
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
        self.deck.cards_in_deck.shuffle(&mut rng);
    }
    fn draw_cards(&mut self, num_cards: i32) {
        if self.deck.cards_in_deck.len() < num_cards as usize {
            self.shuffle_deck();
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
    fn add_mana(&mut self, mana: i32) {
        self.mana += mana;
    }

    fn spend_mana(&mut self, mana: i32) {
        self.mana -= mana;
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.cards_in_hand
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
