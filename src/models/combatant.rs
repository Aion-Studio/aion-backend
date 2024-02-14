use core::fmt;
use std::fmt::Formatter;

use crate::services::traits::combat_decision_maker::DecisionMaker;

use super::talent::Talent;

pub trait Combatant: Send + Sync {
    fn get_id(&self) -> String;
    fn get_name(&self) -> &str;
    fn get_hp(&self) -> i32;
    fn get_damage(&self) -> i32;
    fn get_talents(&self) -> &Vec<Talent>;

    fn attack(&mut self, other: &mut dyn Combatant);
    fn take_damage(&mut self, damage: i32);
}

impl fmt::Debug for dyn Combatant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Combatant: {}", self.get_name())
    }
}
