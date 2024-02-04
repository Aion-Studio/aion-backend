use std::collections::HashMap;
use std::fmt;

use crate::prisma::ResourceEnum;
use rand::Rng;
use serde::de::Visitor;
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Resource {
    Aion,
    Valor,
    NexusOrb,
    StormShard,
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
        resources.insert(Resource::NexusOrb, rng.gen_range(1..10));
        resources
    }
}

impl Serialize for Resource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let resource_string = match self {
            Resource::Aion => "Aion".to_string(),
            Resource::Valor => "Valor".to_string(),
            Resource::NexusOrb => "NexusOrb".to_string(),
            Resource::StormShard => "StormShard".to_string(),
        };

        serializer.serialize_str(&resource_string)
    }
}

impl<'de> Deserialize<'de> for Resource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResourceVisitor;

        impl<'de> Visitor<'de> for ResourceVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an object representing a Resource")
            }

            fn visit_str<E>(self, value: &str) -> Result<Resource, E>
            where
                E: de::Error,
            {
                match value {
                    "Aion" => Ok(Resource::Aion),
                    "Valor" => Ok(Resource::Valor),
                    "NexusOrb" => Ok(Resource::NexusOrb),
                    "StormShard" => Ok(Resource::StormShard),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["Aion", "Valor", "NexusShard"],
                    )),
                }
            }
        }

        deserializer.deserialize_any(ResourceVisitor)
    }
}

impl From<Resource> for ResourceEnum {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::Aion => ResourceEnum::Aion,
            Resource::Valor => ResourceEnum::Valor,
            Resource::NexusOrb => ResourceEnum::NexusOrb,
            Resource::StormShard => ResourceEnum::StormShard, // ... handle the Material case, which might be more involved ...
        }
    }
}
