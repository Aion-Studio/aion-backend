use crate::models::resources::Resource;
use prisma_client_rust::chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::services::tasks::channel::ChannelingAction;
use crate::services::tasks::explore::ExploreAction;

#[derive(Debug, Clone, Serialize)]
pub enum GameEvent {
    HeroExplores(ExploreAction),
    ExploreCompleted(ExploreAction),
    LootBoxCreated(TaskLootBox),
    Channeling(ChannelingAction), // add other kinds of tasks here
    ChannelingCompleted(ChannelingAction),
}

impl GameEvent {
    pub fn hero_explores() -> &'static str {
        "HeroExplores"
    }
    pub fn explore_completed() -> &'static str {
        "ExploreCompleted"
    }

    pub fn channeling() -> &'static str {
        "Channeling"
    }
    pub fn channeling_completed() -> &'static str {
        "ChannelingCompleted"
    }

    pub fn name(&self) -> String {
        match self {
            GameEvent::HeroExplores { .. } => "HeroExplores".to_string(),
            GameEvent::ExploreCompleted { .. } => "ExploreCompleted".to_string(),
            GameEvent::LootBoxCreated { .. } => "LootBoxCreated".to_string(),
            GameEvent::Channeling { .. } => "Channeling".to_string(),
            GameEvent::ChannelingCompleted { .. } => "ChannelingCompleted".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum TaskAction {
    Explore(ExploreAction),
    Channel(ChannelingAction),
    // Quest(QuestAction),
}

#[derive(Debug, Clone, Serialize)]
pub enum TaskLootBox {
    Region(RegionActionResult),
    Channel(ChannelActionResult),
}

impl TaskLootBox {
    pub fn new() -> Self {
        //return a RegionActionResult
        TaskLootBox::Region(RegionActionResult {
            hero_id: "hero_id".to_string(),
            resources: vec![],
            xp: 0,
            discovery_level_increase: 0.0,
            created_time: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChannelActionResult {
    pub hero_id: String,
    pub resources: Vec<Resource>,
    pub xp: i32,
    pub created_time: Option<DateTime<FixedOffset>>,
    pub stamina_gained: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RegionActionResult {
    pub hero_id: String,
    pub resources: Vec<Resource>,
    pub xp: i32,
    pub discovery_level_increase: f64,
    pub created_time: Option<DateTime<FixedOffset>>,
}
