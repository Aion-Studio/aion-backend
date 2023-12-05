use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use crate::{
    configuration::ChannelDurations,
    events::game::{ChannelResult, ExploreResult, QuestResult},
    models::{region::RegionName, quest::Quest},
};

use anyhow::Result;

use super::{channel::ChannelingAction, explore::ExploreAction, off_beat_actions::OffBeatActions};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionNames {
    Explore,
    Channel,
    Quest,
    Raid,
    Unique(OffBeatActions),
}

impl ActionNames {
    pub fn to_string(&self) -> String {
        match self {
            ActionNames::Explore => "Explore".to_string(),
            ActionNames::Channel => "Channel".to_string(),
            ActionNames::Quest => "Quest".to_string(),
            ActionNames::Raid => "Raid".to_string(),
            ActionNames::Unique(OffBeatActions::SlayDragonQuest) => "SlayDragonQuest".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Explore" => Some(ActionNames::Explore),
            "Channel" => Some(ActionNames::Channel),
            "Quest" => Some(ActionNames::Quest),
            "Raid" => Some(ActionNames::Raid),
            // Add other cases as needed
            _ => None,
        }
    }

    pub fn from_string(action_name: &str) -> Self {
        match action_name {
            "Explore" => ActionNames::Explore,
            "Channel" => ActionNames::Channel,
            "Quest" => ActionNames::Quest,
            "Raid" => ActionNames::Raid,
            _ => unreachable!(),
        }
    }
}

pub type Responder<T> = oneshot::Sender<Result<T>>;

#[derive(Debug, Clone, Serialize)]
pub enum TaskAction {
    Explore(ExploreAction),
    Channel(ChannelingAction),
}

#[derive(Debug)]
pub enum Command {
    Explore {
        hero_id: String,
        region_name: RegionName,
        resp: Responder<()>,
    },
    ExploreCompleted(ExploreAction),
    Channel {
        hero_id: String,
        leyline_name: String,
        durations: ChannelDurations,
        resp: Responder<()>,
    },
    ChannelCompleted(ChannelingAction),
    QuestAction {
        hero_id: String,
        action_id: String,
        resp: Responder<()>,
    },
    QuestActionDone(String),
    QuestCompleted(String, Quest),
}

// async-timed actions
impl TaskAction {
    pub fn name(&self) -> String {
        match self {
            TaskAction::Explore { .. } => "Explore".to_string(),
            TaskAction::Channel { .. } => "Channel".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskLootBox {
    Region(ExploreResult),
    Channel(ChannelResult),
    Quest(QuestResult),
}
