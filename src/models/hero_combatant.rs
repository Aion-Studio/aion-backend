use std::any::Any;
use std::cmp::{max, min};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::events::combat::{CombatError, CombatantState};
use crate::models::cards::{Card, Deck};
use crate::models::combatant::Combatant;

use super::hero::Hero;
use super::resources::Relic;
use super::talent::Spell;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeroCombatant {
    id: String,
    pub mana: i32,
    hero: Hero,
    pub cards_in_discard: Vec<Card>,
    pub cards_in_hand: Vec<Card>,
    pub deck: Deck,
    zeal: i32,
    max_hp: i32,
    relics: Vec<Relic>,
}

impl HeroCombatant {
    pub fn new(hero: Hero, deck: Deck, relics: Vec<Relic>) -> Self {
        let max_hp = hero.hp;
        HeroCombatant {
            id: hero.id.clone().unwrap(),
            hero,
            cards_in_discard: vec![],
            cards_in_hand: vec![],
            deck,
            mana: 3,
            zeal: 0,
            relics,
            max_hp,
        }
    }
}

impl Default for HeroCombatant {
    fn default() -> Self {
        let hero = Hero::default();
        let deck = Deck::default();
        let relics = vec![];
        HeroCombatant::new(hero, deck, relics)
    }
}

impl Combatant for HeroCombatant {
    fn get_player_state(&self) -> CombatantState {
        CombatantState::Player {
            max_hp: self.max_hp,
            hp: self.get_hp(),
            mana: self.mana,
            zeal: self.zeal,
            armor: self.get_armor(),
            strength: self.hero.strength,
            intelligence: self.hero.intelligence,
            dexterity: self.hero.dexterity,
            spells: self.get_spells(),
            relics: self.get_relics(),
            drawn_cards: self.cards_in_hand.clone(),
            cards_in_discard: self.cards_in_discard.clone(),
        }
    }

    fn heal(&mut self, amount: i32) {
        self.hero.hp = min(self.max_hp, self.hero.hp + amount);
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_name(&self) -> &str {
        &self.hero.name
    }

    fn get_cards_in_discard(&self) -> &Vec<Card> {
        &self.cards_in_discard
    }

    fn get_zeal(&self) -> i32 {
        self.zeal
    }

    fn get_hp(&self) -> i32 {
        self.hero.hp
    }

    fn get_spells(&self) -> Vec<Spell> {
        self.hero.spells.clone()
    }

    fn get_relics(&self) -> Vec<Relic> {
        self.relics.clone()
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

    // put back remaining in-hand cards + discard pile back to deck and shuffle
    fn shuffle_deck(&mut self) {
        self.deck.cards_in_deck.append(&mut self.cards_in_hand);
        self.deck.cards_in_deck.append(&mut self.cards_in_discard);
        self.cards_in_hand.clear();
        self.cards_in_discard.clear();

        let deck = &mut self.deck;
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
        let mut random_cards_from_deck = (0..num_cards_to_draw)
            .map(|_| {
                let random_index = rand::random::<usize>() % self.deck.cards_in_deck.len();
                let card = self.deck.cards_in_deck.remove(random_index);
                card
            })
            .collect::<Vec<Card>>();

        self.cards_in_hand.append(&mut random_cards_from_deck);
        println!("drew cards for hero");
    }

    fn add_to_discard(&mut self, card: Card) {
        let card_id = card.id.clone();
        self.cards_in_discard.push(card);
        self.cards_in_hand.retain(|c| c.id != card_id);
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
            self.add_to_discard(card.clone());
            Ok(())
        } else {
            Err(CombatError::CardNotInHand)
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
