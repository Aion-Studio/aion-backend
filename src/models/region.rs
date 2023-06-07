use super::resources::Resource;
use prisma_client_rust::chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub struct HeroRegion {
    pub hero_id: String,
    pub region_name: RegionName,
    pub discovery_level: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub name: RegionName,
    pub leylines: Vec<Leyline>,
    pub adjacent_regions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct RegionActionResult {
    pub resources: Vec<Resource>,
    pub xp: i32,
    pub discovery_level_increase: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RegionName {
    Dusane,
    Yezer,
    Emerlad,
    Forest,
    Buzna,
}
