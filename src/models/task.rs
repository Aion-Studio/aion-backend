use prisma_client_rust::chrono::{DateTime, FixedOffset};
use serde::Serialize;

use crate::services::tasks::explore::ExploreAction;

use super::resources::Resource;


#[derive(Debug, Clone, Serialize)]
pub enum TaskKind {
    Exploration(ExploreAction),
    // add other kinds of tasks here
}

#[derive(Debug, Clone, Serialize)]
pub enum TaskAction {
    Explore(ExploreAction),
    // Quest(QuestAction),
}

#[derive(Debug, Clone, Serialize)]
pub enum TaskLootBox {
    Region(RegionActionResult),
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
pub struct RegionActionResult {
    pub hero_id: String,
    pub resources: Vec<Resource>,
    pub xp: i32,
    pub discovery_level_increase: f64,
    pub created_time: Option<DateTime<FixedOffset>>,
}
