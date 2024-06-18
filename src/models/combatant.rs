use core::fmt;
use std::any::Any;
use std::fmt::Formatter;

use crate::events::combat::CombatError;
use crate::models::cards::Card;

use super::talent::Spell;

// pub trait Combatant: Send + Sync {
pub trait Combatant: CloneBoxCombatant + Send + Sync {
    fn get_id(&self) -> String;
    fn get_name(&self) -> &str;
    fn get_hp(&self) -> i32;
    fn get_damage(&self) -> i32;
    fn get_mana(&self) -> i32;
    fn get_spells(&self) -> Vec<Spell>;

    fn get_armor(&self) -> i32;
    fn get_level(&self) -> i32;

    // fn attack(&self, other: &mut dyn Combatant);
    fn take_damage(&mut self, amount: i32);
    fn shuffle_deck(&mut self);
    fn draw_cards(&mut self); // goes from deck to hand
    fn add_to_discard(&mut self, card: Card);
    /// Sets the hero's mana to the amount
    fn add_mana(&mut self);
    fn spend_mana(&mut self, mana: i32);
    fn get_hand(&self) -> &Vec<Card>;
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
