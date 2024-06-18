use std::any::Any;
use std::cmp::max;

use serde::{Deserialize, Serialize};

use crate::events::combat::CombatError;
use crate::models::cards::{Card, Deck};
use crate::models::combatant::Combatant;

use super::hero::Hero;
use super::talent::Spell;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeroCombatant {
    id: String,
    mana: i32,
    hero: Hero,
    cards_in_discard: Vec<Card>,
    cards_in_hand: Vec<Card>,
    deck: Deck,
    gauge: i32,
}

impl HeroCombatant {
    pub fn new(hero: Hero, deck: Deck) -> Self {
        HeroCombatant {
            id: hero.id.clone().unwrap(),
            hero,
            cards_in_discard: vec![],
            cards_in_hand: vec![],
            deck,
            mana: 0,
            gauge: 0,
        }
    }
}
impl Combatant for HeroCombatant {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> &str {
        &self.hero.name
    }
    fn get_hp(&self) -> i32 {
        self.hero.hp
    }

    fn get_damage(&self) -> i32 {
        self.hero.strength
    }

    fn get_spells(&self) -> Vec<Spell> {
        self.hero.spells.clone()
    }

    fn get_armor(&self) -> i32 {
        self.hero.armor
    }

    fn get_level(&self) -> i32 {
        self.hero.level
    }

    fn take_damage(&mut self, damage: i32) {
        self.hero.hp = max(0, self.hero.hp - damage);
    }

    fn get_mana(&self) -> i32 {
        self.mana
    }

    fn add_mana(&mut self) {
        // NOTE: eventually this will be a variable
        self.mana += 3;
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

    fn draw_cards(&mut self) {
        let num_cards_to_draw = 5; // NOTE: evenutally effects can make this a variable
        if self.deck.cards_in_deck.len() < num_cards_to_draw as usize {
            self.shuffle_deck();
        }
        if self.deck.cards_in_deck.is_empty() {
            return;
        }
        self.cards_in_hand.append(
            &mut self
                .deck
                .cards_in_deck
                .drain(0..num_cards_to_draw as usize)
                .collect::<Vec<Card>>(),
        );
    }

    fn add_to_discard(&mut self, card: Card) {
        self.cards_in_discard.push(card);
    }
    /// Sets the hero's mana to the amount

    fn spend_mana(&mut self, energy: i32) {
        self.mana -= energy;
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.cards_in_hand
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
