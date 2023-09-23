use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Resource {
    Aion,
    Valor,
    NexusShard,
    Oak,
    IronOre,
    Copper,
    Silk,
}

#[derive(Clone, Debug)]
pub struct ResourceCost {
    resource_type: Resource,
    amount: i32,
}
