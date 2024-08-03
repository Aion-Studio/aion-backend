use core::fmt;
use std::any::Any;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::events::combat::{CombatError, CombatantState};
use crate::models::cards::Card;

use super::hero_combatant::HeroCombatant;
use super::npc::Monster;
use super::resources::Relic;
use super::talent::Spell;

// pub trait Combatant: Send + Sync {
pub trait Combatant: CloneBoxCombatant + Send + Sync {
    fn get_id(&self) -> String;
    fn get_name(&self) -> &str;
    fn get_hp(&self) -> i32;
    fn get_mana(&self) -> i32;
    fn get_spells(&self) -> Vec<Spell>;
    fn heal(&mut self, amount: i32);

    fn get_armor(&self) -> i32;
    fn get_level(&self) -> i32;
    fn get_player_state(&self) -> CombatantState;

    // fn attack(&self, other: &mut dyn Combatant);
    fn take_damage(&mut self, amount: i32, is_chaos: bool);
    fn shuffle_deck(&mut self);
    fn draw_cards(&mut self); // goes from deck to hand
    fn add_to_discard(&mut self, card: Card);
    fn get_cards_in_discard(&self) -> &Vec<Card>;
    fn get_zeal(&self) -> i32;
    /// Sets the hero's mana to the amount
    fn add_mana(&mut self);
    fn boost_mana(&mut self, amount: i32);
    fn spend_mana(&mut self, mana: i32);
    fn get_hand(&self) -> &Vec<Card>;
    fn get_relics(&self) -> Vec<Relic>;
    /// Removes a card from hand.
    /// Caller is responsible for moving card to battlefield
    fn play_card(&mut self, card: &Card) -> Result<(), CombatError>;
    fn as_any(&self) -> &dyn Any;
}

impl fmt::Debug for dyn Combatant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Combatant: {}", self.get_name())
    }
}

pub trait CloneBoxCombatant {
    fn clone_box(&self) -> Box<dyn Combatant>;
}
// Implement this trait for any type that implements `Combatant` + `Clone`
impl<T> CloneBoxCombatant for T
where
    T: 'static + Combatant + Clone,
{
    fn clone_box(&self) -> Box<dyn Combatant> {
        Box::new(self.clone())
    }
}

// Define an enum to represent different types of combatants
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CombatantType {
    Hero(HeroCombatant),
    Monster(Monster),
}

impl CombatantType {
    pub fn as_combatant(&self) -> &dyn Combatant {
        match self {
            CombatantType::Hero(hero) => hero,
            CombatantType::Monster(monster) => monster,
        }
    }

    pub fn as_hero(&mut self) -> &mut HeroCombatant {
        match self {
            CombatantType::Hero(hero) => hero,
            CombatantType::Monster(_) => panic!("Expected hero"),
        }
    }

    pub fn as_monster(&mut self) -> &mut Monster {
        match self {
            CombatantType::Monster(monster) => monster,
            CombatantType::Hero(_) => panic!("Expected monster"),
        }
    }

    pub fn as_combatant_mut(&mut self) -> &mut dyn Combatant {
        match self {
            CombatantType::Hero(hero) => hero,
            CombatantType::Monster(monster) => monster,
        }
    }
}
