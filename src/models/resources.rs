use std::collections::HashMap;

use crate::prisma::{CommonEnum, EpicEnum, RareEnum, ResourceEnum};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Resource {
    Aion,
    Valor,
    NexusShard,
    Material(MaterialType),
}

#[derive(Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum MaterialType {
    Common(Common),
    Rare(Rare),
    Epic(Epic),
}

#[derive(Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Common {
    IronOre,
    RoughLeather,
    Quartz,
}

#[derive(Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Rare {
    SilverOre,
    FineLeather,
    Sapphire,
}

#[derive(Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Epic {
    MythrilOre,
    Dragonhide,
    Ruby,
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
        resources
    }
}
impl From<CommonEnum> for Common {
    fn from(prisma_enum: CommonEnum) -> Self {
        match prisma_enum {
            CommonEnum::IronOre => Common::IronOre,
            CommonEnum::RoughLeather => Common::RoughLeather,
            CommonEnum::Quartz => Common::Quartz,
        }
    }
}
impl From<RareEnum> for Rare {
    fn from(prisma_enum: RareEnum) -> Self {
        match prisma_enum {
            RareEnum::SilverOre => Rare::SilverOre,
            RareEnum::FineLeather => Rare::FineLeather,
            RareEnum::Sapphire => Rare::Sapphire,
        }
    }
}

impl From<EpicEnum> for Epic {
    fn from(prisma_enum: EpicEnum) -> Self {
        match prisma_enum {
            EpicEnum::MythrilOre => Epic::MythrilOre,
            EpicEnum::Dragonhide => Epic::Dragonhide,
            EpicEnum::Ruby => Epic::Ruby,
        }
    }
}
impl From<Resource> for ResourceEnum {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::Aion => ResourceEnum::Aion,
            Resource::Valor => ResourceEnum::Valor,
            Resource::NexusShard => ResourceEnum::NexusShard,
            Resource::Material(material) => ResourceEnum::Material, // ... handle the Material case, which might be more involved ...
        }
    }
}
