use std::str::FromStr;

use super::resources::Resource;
use prisma_client_rust::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HeroRegion {
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
    pub location: String,
    pub xp_reward: i32,
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

pub enum TaskResult {
    Region(RegionActionResult),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegionActionResult {
    pub hero_id: String,
    pub resources: Vec<Resource>,
    pub xp: i32,
    pub discovery_level_increase: f64,
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
