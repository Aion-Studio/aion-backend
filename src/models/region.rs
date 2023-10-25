use crate::prisma::hero_region::SetParam;
use lazy_static::lazy_static;
use prisma_client_rust::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

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

impl Leyline {
    pub fn new(
        name: String,
        xp_reward: i32,
        region_name: RegionName,
        discovery_required: i32,
        stamina_rate: f64,
        aion_rate: f64,
    ) -> Self {
        Self {
            name,
            xp_reward,
            region_name,
            discovery_required,
            stamina_rate,
            aion_rate,
        }
    }
}

lazy_static! {
    pub static ref DUSAWATER: Leyline = Leyline::new(
        "Dusawater".to_string(),
        9,
        RegionName::Dusane,
        2,
        40.0,
        8.0
    );
    // ... Define other leylines similarly ...
    pub static ref DUSAEARTH: Leyline = Leyline::new(
        "Dusaearth".to_string(),
        8,
        RegionName::Dusane,
        10,
        41.0,
        3.0
    );

    pub static ref DUSAGLOW: Leyline = Leyline::new(
        "Dusaglow".to_string(),
        10,
        RegionName::Dusane,
        20,
        42.0,
        2.0
    );

    pub static ref DUSAFIRE: Leyline = Leyline::new(
        "Dusafire".to_string(),
        11,
        RegionName::Dusane,
        30,
        43.0,
        1.0
    );
    pub static ref DUSADREAM: Leyline = Leyline::new(
        "Dusadream".to_string(),
        12,
        RegionName::Dusane,
        40,
        44.0,
        0.5
    );
    pub static ref DUSACLOUD: Leyline = Leyline::new(
        "Dusacloud".to_string(),
        13,
        RegionName::Dusane,
        50,
        45.0,
        0.25
    );
    pub static ref DUSAWIND: Leyline = Leyline::new(
        "Dusawind".to_string(),
        14,
        RegionName::Dusane,
        60,
        46.0,
        0.125
    );

    pub static  ref DUSALIGHT: Leyline = Leyline::new(
        "Dusalight".to_string(),
        15,
        RegionName::Dusane,
        70,
        47.0,
        0.0625
    );

    pub static  ref  DUSAPARK: Leyline = Leyline::new(
        "Dusapark".to_string(),
        16,
        RegionName::Dusane,
        80,
        48.0,
        0.03125
    );

    pub static ref DUSAROCK: Leyline = Leyline::new(
        "Dusarock".to_string(),
        17,
        RegionName::Dusane,
        90,
        49.0,
        0.015625
    );
    // ... and so on for each leyline ...
}

lazy_static! {
    pub static ref LEYLINES: Vec<Leyline> = vec![
        DUSAWATER.clone(),
        DUSAEARTH.clone(),
        DUSAGLOW.clone(),
        DUSAFIRE.clone(),
        DUSADREAM.clone(),
        DUSACLOUD.clone(),
        DUSAWIND.clone(),
        DUSALIGHT.clone(),
        DUSAPARK.clone(),
        DUSAROCK.clone(),
        // ... Add other leylines ...
    ];
}

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
    pub fn max_discovery(&self) -> f64 {
        match self {
            RegionName::Dusane => 150.0,
            RegionName::Yezer => 178.0,
            RegionName::Emerlad => 110.0,
            RegionName::Forest => 243.0,
            RegionName::Buzna => 341.0,
            RegionName::Veladria => 287.0,
            RegionName::Lindon => 412.0,
        }
    }
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
