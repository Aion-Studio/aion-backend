use std::collections::HashMap;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::prisma::{relic, EffectType, Resource as ResourceEnum, TargetType};
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Relic {
    pub id: String,
    pub name: String,
    pub effects: Vec<RelicEffect>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RelicEffect {
    pub value: i32,
    pub effect_type: EffectType,
    pub target: TargetType,
    pub duration: i32,
}

impl From<relic::Data> for Relic {
    fn from(relic: relic::Data) -> Self {
        Relic {
            id: relic.id,
            name: relic.name,
            effects: relic
                .effects
                .map(|effects| {
                    effects
                        .into_iter()
                        .map(|effect| RelicEffect {
                            value: effect.value,
                            effect_type: effect.effect_type.clone(),
                            target: effect.target.clone(),
                            duration: match effect.duration {
                                Some(duration) => duration,
                                None => 0,
                            },
                        })
                        .collect::<Vec<RelicEffect>>()
                })
                .unwrap_or_default(),
        }
    }
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
