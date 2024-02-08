use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

use crate::{
    configuration::ChannelDurations,
    events::game::{ChannelResult, ExploreResult, QuestResult},
    models::{quest::Quest, region::RegionName},
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
            ActionNames::Unique(x) => match x {
                OffBeatActions::SlayDragonQuest => "SlayDragonQuest".to_string(),
                _ => unreachable!(),
            },
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
    QuestAction(String, String),
    QuestComplete(String, Quest),
    QuestAccepted(String, String),
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
    QuestActionDone(String, String),
    QuestCompleted(String, Quest),
    QuestAccepted {
        hero_id: String,
        quest_id: String,
        resp: Responder<()>,
    },
}

// async-timed actions
impl TaskAction {
    pub fn name(&self) -> String {
        match self {
            TaskAction::Explore { .. } => "Explore".to_string(),
            TaskAction::Channel { .. } => "Channel".to_string(),
            TaskAction::QuestAction(_, _) => "QuestAction".to_string(),
            TaskAction::QuestComplete(_, _) => "QuestComplete".to_string(),
            TaskAction::QuestAccepted(_, _) => "QuestAccepted".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskLootBox {
    Region(ExploreResult),
    Channel(ChannelResult),
    Quest(QuestResult),
}

impl TaskLootBox {
    pub fn name(&self) -> String {
        match self {
            TaskLootBox::Region(_) => "explore-lootbox".to_string(),
            TaskLootBox::Channel(_) => "channel-lootbox".to_string(),
            TaskLootBox::Quest(_) => "quest-lootbox".to_string(),
        }
    }
}
