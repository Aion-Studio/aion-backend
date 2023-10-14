use std::collections::HashMap;

use rand::Rng;
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

impl Resource {
    pub fn randomize_amounts() -> HashMap<Resource, i32> {
        let mut rng = rand::thread_rng();
        let mut resources = HashMap::new();
        resources.insert(Resource::Aion, rng.gen_range(1..10));
        resources.insert(Resource::Valor, rng.gen_range(1..10));
        resources.insert(Resource::NexusShard, rng.gen_range(1..10));
        resources.insert(Resource::Oak, rng.gen_range(1..10));
        resources.insert(Resource::IronOre, rng.gen_range(1..10));
        resources.insert(Resource::Copper, rng.gen_range(1..10));
        resources.insert(Resource::Silk, rng.gen_range(1..10));
        resources
    }
}
