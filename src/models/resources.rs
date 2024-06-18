use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::prisma::Resource as ResourceEnum;
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    Aion,
    Flux,
    Gem,
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
        resources
    }
}

impl From<Resource> for ResourceEnum {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::Aion => ResourceEnum::Aion,
            Resource::Flux => ResourceEnum::Flux,
            Resource::Gem => ResourceEnum::Gem,
        }
    }
}
