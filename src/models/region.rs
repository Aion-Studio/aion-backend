use std::collections::HashMap;
use std::str::FromStr;

use prisma_client_rust::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::prisma::hero_region::SetParam;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeroRegion {
    pub id: Option<String>,
    pub hero_id: String,
    pub region_name: RegionName,
    pub discovery_level: i32,
    pub current_location: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub name: RegionName,
    pub leylines: Vec<Leyline>,
    pub adjacent_regions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Leyline {
    pub name: String,
    pub xp_reward: i32,
    pub region_name: RegionName,
    pub discovery_required: i32,
    pub stamina_rate: f64,
    pub aion_rate: f64,
}

pub const DUSAWATER: Leyline = Leyline {
    name: "Dusawater".to_string(),
    xp_reward: 9,
    region_name: RegionName::Dusane,
    discovery_required: 2,
    stamina_rate: 40.0,
    aion_rate: 8.0,
};

pub const DUSAEARTH: Leyline = Leyline {
    name: "Dusaearth".to_string(),
    xp_reward: 8,
    region_name: RegionName::Dusane,
    discovery_required: 10,
    stamina_rate: 41.0,
    aion_rate: 3.0,
};

pub const DUSAGLOW: Leyline = Leyline {
    name: "Dusaglow".to_string(),
    xp_reward: 14,
    region_name: RegionName::Dusane,
    discovery_required: 4,
    stamina_rate: 22.0,
    aion_rate: 8.0,
};

pub const DUSADREAM: Leyline = Leyline {
    name: "Dusadream".to_string(),
    xp_reward: 12,
    region_name: RegionName::Dusane,
    discovery_required: 7,
    stamina_rate: 15.0,
    aion_rate: 3.0,
};

pub const DUSAWIND: Leyline = Leyline {
    name: "Dusawind".to_string(),
    xp_reward: 17,
    region_name: RegionName::Dusane,
    discovery_required: 4,
    stamina_rate: 85.0,
    aion_rate: 4.0,
};

pub const DUSACLOUD: Leyline = Leyline {
    name: "Dusacloud".to_string(),
    xp_reward: 11,
    region_name: RegionName::Dusane,
    discovery_required: 7,
    stamina_rate: 72.0,
    aion_rate: 4.0,
};

pub const DUSASPARK: Leyline = Leyline {
    name: "Dusaspark".to_string(),
    xp_reward: 6,
    region_name: RegionName::Dusane,
    discovery_required: 2,
    stamina_rate: 61.0,
    aion_rate: 9.0,
};

pub const DUSAFIRE: Leyline = Leyline {
    name: "Dusafire".to_string(),
    xp_reward: 11,
    region_name: RegionName::Dusane,
    discovery_required: 6,
    stamina_rate: 45.0,
    aion_rate: 5.0,
};

pub const DUSALIGHT: Leyline = Leyline {
    name: "Dusalight".to_string(),
    xp_reward: 3,
    region_name: RegionName::Dusane,
    discovery_required: 5,
    stamina_rate: 5.0,
    aion_rate: 6.0,
};

pub const DUSAROCK: Leyline = Leyline {
    name: "Dusarock".to_string(),
    xp_reward: 10,
    region_name: RegionName::Dusane,
    discovery_required: 8,
    stamina_rate: 0.0,
    aion_rate: 4.0,
};

pub const LEYLINES: [Leyline; 10] = [
    DUSAWATER, DUSAEARTH, DUSAGLOW, DUSADREAM, DUSAWIND, DUSACLOUD, DUSASPARK, DUSAFIRE, DUSALIGHT,
    DUSAROCK,
];

pub fn leyline_map() -> HashMap<String, Leyline> {
    let mut map = HashMap::new();
    for leyline in LEYLINES.iter() {
        map.insert(leyline.name.clone(), leyline.clone());
    }
    map
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegionAction {
    pub hero_id: String,
    pub region_name: RegionName,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub enum ActionError {
    InternalError(String),
    RegionActionError,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RegionName {
    Dusane,
    Yezer,
    Emerlad,
    Forest,
    Buzna,
    Veladria,
    Lindon,
}

impl RegionName {
    // random() returns a random variant of RegionName
    pub(crate) fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let variants = vec![
            RegionName::Dusane,
            RegionName::Yezer,
            RegionName::Emerlad,
            RegionName::Forest,
            RegionName::Buzna,
            RegionName::Veladria,
            RegionName::Lindon,
        ];
        let index = rng.gen_range(0..variants.len());
        variants[index].clone()
    }
    pub(crate) fn to_str(&self) -> String {
        match self {
            RegionName::Dusane => "Dusane".to_string(),
            RegionName::Yezer => "Yezer".to_string(),
            RegionName::Emerlad => "Emerlad".to_string(),
            RegionName::Forest => "Forest".to_string(),
            RegionName::Buzna => "Buzna".to_string(),
            RegionName::Veladria => "Veladria".to_string(),
            RegionName::Lindon => "Lindon".to_string(),
        }
    }
}

// converts a string into a RegionName via str.parse().unwrap
impl FromStr for RegionName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dusane" => Ok(RegionName::Dusane),
            "Yezer" => Ok(RegionName::Yezer),
            "Emerlad" => Ok(RegionName::Emerlad),
            "Forest" => Ok(RegionName::Forest),
            "Buzna" => Ok(RegionName::Buzna),
            "Veladria" => Ok(RegionName::Veladria),
            "Lindon" => Ok(RegionName::Lindon),
            _ => Err(()),
        }
    }
}

impl HeroRegion {
    pub fn set(hero_region: &HeroRegion) -> Vec<SetParam> {
        vec![
            SetParam::SetHeroId(hero_region.hero_id.clone()),
            SetParam::SetRegionName(hero_region.region_name.clone().to_str()),
            SetParam::SetDiscoveryLevel(hero_region.discovery_level),
            SetParam::SetCurrentLocation(hero_region.current_location),
        ]
    }
}
