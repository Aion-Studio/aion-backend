use core::fmt;
use std::any::Any;
use std::fmt::Formatter;

use crate::services::traits::combat_decision_maker::DecisionMaker;

use super::talent::Talent;

// pub trait Combatant: Send + Sync {
pub trait Combatant: CloneBoxCombatant + Send + Sync {
    fn get_id(&self) -> String;
    fn get_name(&self) -> &str;
    fn get_hp(&self) -> i32;
    fn get_damage(&self) -> i32;
    fn get_talents(&self) -> &Vec<Talent>;

    fn attack(&mut self, other: &mut dyn Combatant);
    fn take_damage(&mut self, damage: i32);
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
