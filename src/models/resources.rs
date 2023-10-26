use derive_more::Display;
use std::collections::HashMap;
use std::fmt;

use crate::models::hero::Hero;
use crate::prisma::{CommonEnum, EpicEnum, MaterialEnum, RareEnum, ResourceEnum};
use crate::services::tasks::explore::ExploreAction;
use rand::Rng;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Resource {
    Aion,
    Valor,
    NexusShard,
    Material(MaterialType),
}

#[derive(Display, Clone, Debug, Hash, Deserialize, Eq, PartialEq)]
pub enum MaterialType {
    Common(Common),
    Rare(Rare),
    Epic(Epic),
}

#[derive(Display, Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Common {
    IronOre,
    RoughLeather,
    Quartz,
}

#[derive(Display, Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
pub enum Rare {
    SilverOre,
    FineLeather,
    Sapphire,
}

#[derive(Display, Clone, Debug, Serialize, Hash, Deserialize, Eq, PartialEq)]
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

impl MaterialType {
    pub fn get_common_rng() -> MaterialType {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);
        let c = match roll {
            0..=39 => Common::IronOre,       // 40% chance for IronOre
            40..=79 => Common::RoughLeather, // 40% chance for RoughLeather
            80..=99 => Common::Quartz,       // 20% chance for Quartz
            _ => unreachable!(),             // This case will never be reached due to our range
        };
        MaterialType::Common(c)
    }

    pub fn get_maybe_rare(
        hero: &Hero,
        explore_action: &ExploreAction,
        discovery: f64,
    ) -> MaterialType {
        let mut rng = rand::thread_rng();
        let discovery_factor = discovery / explore_action.region_name.max_discovery();
        let base_chance = (hero.level() - 10) as f64 * 0.4f64;
        let rdf_bonus = base_chance * discovery_factor;
        let chance_of_rare = base_chance + rdf_bonus;
        let random_value: f64 = rng.gen();

        // Check if the random value is less than or equal to the chance_of_rare
        if random_value <= chance_of_rare {
            // Rare event occurs
            let roll = rng.gen_range(0..100);
            let r = match roll {
                0..=39 => Rare::SilverOre,    // 40% chance for SilverOre
                40..=79 => Rare::FineLeather, // 40% chance for FineLeather
                80..=99 => Rare::Sapphire,    // 20% chance for Sapphire
                _ => unreachable!(),          // This case will never be reached due to our range
            };
            MaterialType::Rare(r)
        } else {
            // Rare event does not occur
            MaterialType::get_common_rng()
        }
    }

    pub fn get_maybe_epic(
        hero: &Hero,
        explore_action: &ExploreAction,
        discovery: f64,
    ) -> MaterialType {
        let discovery_factor = discovery / explore_action.region_name.max_discovery();
        let base_chance = (hero.level() - 30) as f64 * 0.001;
        let rdf_bonus = base_chance * discovery_factor * 0.5;
        let chance_of_epic = base_chance + rdf_bonus;
        let random_value: f64 = rand::thread_rng().gen();

        if random_value <= chance_of_epic {
            // Epic event occurs
            let mut rng = rand::thread_rng();
            let roll = rng.gen_range(0..100);
            let e = match roll {
                0..=39 => Epic::MythrilOre,  // 40% chance for MythrilOre
                40..=79 => Epic::Dragonhide, // 40% chance for Dragonhide
                80..=99 => Epic::Ruby,       // 20% chance for Ruby
                _ => unreachable!(),         // This case will never be reached due to our range
            };
            MaterialType::Epic(e)
        } else {
            // Epic event does not occur
            MaterialType::get_maybe_rare(hero, explore_action, discovery)
        }
    }
}
// impl Serialize for MaterialType {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             MaterialType::Common(common) => {
//                 let mut state = serializer.serialize_struct("MaterialType", 1)?;
//                 state.serialize_field("Common", &common.to_string())?;
//                 state.end()
//             }
//             MaterialType::Rare(rare) => {
//                 let mut state = serializer.serialize_struct("MaterialType", 1)?;
//                 state.serialize_field("Rare", &rare.to_string())?;
//                 state.end()
//             }
//             MaterialType::Epic(epic) => {
//                 let mut state = serializer.serialize_struct("MaterialType", 1)?;
//                 state.serialize_field("Epic", &epic.to_string())?;
//                 state.end()
//             }
//         }
//     }
// }

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

impl Serialize for Resource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Resource::Aion => serializer.serialize_str("Aion"),
            Resource::Valor => serializer.serialize_str("Valor"),
            Resource::NexusShard => serializer.serialize_str("NexusShard"),
            Resource::Material(material) => {
                let mut map = serializer.serialize_map(Some(1))?;
                match material {
                    MaterialType::Common(common) => {
                        map.serialize_entry("Common", &common.to_string())?;
                    }
                    MaterialType::Rare(rare) => {
                        map.serialize_entry("Rare", &rare.to_string())?;
                    }
                    MaterialType::Epic(epic) => {
                        map.serialize_entry("Epic", &epic.to_string())?;
                    }
                }
                map.end()
            }
        }
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
                    "NexusShard" => Ok(Resource::NexusShard),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["Aion", "Valor", "NexusShard"],
                    )),
                }
            }

            fn visit_map<M>(self, mut access: M) -> Result<Resource, M::Error>
            where
                M: MapAccess<'de>,
            {
                if let Some(key) = access.next_key::<String>()? {
                    match key.as_str() {
                        "Common" => {
                            let val: String = access.next_value()?;
                            match val.as_str() {
                                "IronOre" => {
                                    Ok(Resource::Material(MaterialType::Common(Common::IronOre)))
                                }
                                "RoughLeather" => Ok(Resource::Material(MaterialType::Common(
                                    Common::RoughLeather,
                                ))),
                                "Quartz" => {
                                    Ok(Resource::Material(MaterialType::Common(Common::Quartz)))
                                }
                                _ => Err(de::Error::unknown_variant(
                                    &val,
                                    &["IronOre", "RoughLeather", "Quartz"],
                                )),
                            }
                        }
                        "Rare" => {
                            let val: String = access.next_value()?;
                            match val.as_str() {
                                "SilverOre" => {
                                    Ok(Resource::Material(MaterialType::Rare(Rare::SilverOre)))
                                }
                                "FineLeather" => {
                                    Ok(Resource::Material(MaterialType::Rare(Rare::FineLeather)))
                                }
                                "Sapphire" => {
                                    Ok(Resource::Material(MaterialType::Rare(Rare::Sapphire)))
                                }
                                _ => Err(de::Error::unknown_variant(
                                    &val,
                                    &["SilverOre", "FineLeather", "Sapphire"],
                                )),
                            }
                        }
                        "Epic" => {
                            let val: String = access.next_value()?;
                            match val.as_str() {
                                "MythrilOre" => {
                                    Ok(Resource::Material(MaterialType::Epic(Epic::MythrilOre)))
                                }
                                "Dragonhide" => {
                                    Ok(Resource::Material(MaterialType::Epic(Epic::Dragonhide)))
                                }
                                "Ruby" => Ok(Resource::Material(MaterialType::Epic(Epic::Ruby))),
                                _ => Err(de::Error::unknown_variant(
                                    &val,
                                    &["MythrilOre", "Dragonhide", "Ruby"],
                                )),
                            }
                        }
                        _ => Err(de::Error::unknown_field(&key, &["Common", "Rare", "Epic"])),
                    }
                } else {
                    Err(de::Error::invalid_type(de::Unexpected::Map, &self))
                }
            }
        }

        deserializer.deserialize_any(ResourceVisitor)
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
            Resource::Material(_) => ResourceEnum::Material, // ... handle the Material case, which might be more involved ...
        }
    }
}
impl From<MaterialType> for MaterialEnum {
    fn from(material: MaterialType) -> Self {
        match material {
            MaterialType::Common(_) => MaterialEnum::Common,
            MaterialType::Rare(_) => MaterialEnum::Rare,
            MaterialType::Epic(_) => MaterialEnum::Epic,
        }
    }
}
impl From<Common> for CommonEnum {
    fn from(common: Common) -> Self {
        match common {
            Common::IronOre => CommonEnum::IronOre,
            Common::RoughLeather => CommonEnum::RoughLeather,
            Common::Quartz => CommonEnum::Quartz,
        }
    }
}

impl From<Rare> for RareEnum {
    fn from(rare: Rare) -> Self {
        match rare {
            Rare::SilverOre => RareEnum::SilverOre,
            Rare::FineLeather => RareEnum::FineLeather,
            Rare::Sapphire => RareEnum::Sapphire,
        }
    }
}

impl From<Epic> for EpicEnum {
    fn from(epic: Epic) -> Self {
        match epic {
            Epic::MythrilOre => EpicEnum::MythrilOre,
            Epic::Dragonhide => EpicEnum::Dragonhide,
            Epic::Ruby => EpicEnum::Ruby,
        }
    }
}
