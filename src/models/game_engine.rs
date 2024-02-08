use std::time::Duration;

use super::{
    hero::{AttributeModifier, Item},
    resources::ResourceCost,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Attack,
    Defend,
    Harvest,
    Explore,
    Craft,
    SpecialAbility,
    // Add more as needed...
}

#[derive(Clone, Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub cost: Vec<ResourceCost>, // Each action may cost multiple types of resources
    pub duration: Duration,
    pub timeout: Duration,
    pub xp_change: i32,     // positive for gain, negative for loss
    pub health_change: i32, // positive for gain, negative for loss
    pub attribute_modifiers: Vec<AttributeModifier>,
    pub created_items: Vec<Item>, // items created as a result of this action
}
