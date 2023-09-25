use prisma_client_rust::chrono;
use std::collections::HashMap;

use crate::models::hero::Hero;
use crate::models::resources::Resource;
use crate::prisma::action_completed;
use prisma_client_rust::chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::services::tasks::channel::ChannelingAction;
use crate::services::tasks::explore::ExploreAction;

#[derive(Debug, Clone, Serialize)]
pub enum GameEvent {
    HeroExplores(ExploreAction),
    ExploreCompleted(ExploreAction),
    LootBoxCreated(TaskLootBox),
    Channeling(ChannelingAction),
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
}

impl TaskAction {
    pub fn name(&self) -> String {
        match self {
            TaskAction::Explore { .. } => "Explore".to_string(),
            TaskAction::Channel { .. } => "Channel".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum TaskLootBox {
    Region(ExploreResult),
    Channel(ChannelResult),
}

impl Default for TaskLootBox {
    fn default() -> Self {
        TaskLootBox::new()
    }
}

impl TaskLootBox {
    pub fn new() -> Self {
        //return a RegionActionResult
        TaskLootBox::Region(ExploreResult {
            hero_id: "hero_id".to_string(),
            resources: HashMap::new(),
            xp: 0,
            discovery_level_increase: 0.0,
            created_time: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChannelResult {
    pub hero_id: String,
    pub resources: HashMap<Resource, i32>,
    pub xp: i32,
    pub created_time: Option<DateTime<FixedOffset>>,
    pub stamina_gained: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExploreResult {
    pub hero_id: String,
    pub resources: HashMap<Resource, i32>,
    pub xp: i32,
    pub discovery_level_increase: f64,
    pub created_time: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionCompleted {
    pub id: String,
    #[serde(rename = "actionName")]
    pub action_name: String,
    #[serde(rename = "heroId")]
    pub hero_id: String,
    #[serde(rename = "hero")]
    pub hero: Option<Hero>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
}

impl ActionCompleted {
    pub fn new(action_name: String, hero_id: String) -> Self {
        let date = chrono::offset::Utc::now().into();
        ActionCompleted {
            id: uuid::Uuid::new_v4().to_string(),
            action_name,
            hero_id,
            hero: None,
            updated_at: date,
            created_at: date,
        }
    }
}

impl From<action_completed::Data> for ActionCompleted {
    fn from(action_completed: action_completed::Data) -> Self {
        ActionCompleted {
            id: action_completed.id,
            action_name: action_completed.action_name,
            hero_id: action_completed.hero_id,
            hero: None,
            updated_at: action_completed.updated_at,
            created_at: action_completed.created_at,
        }
    }
}
