use anyhow::Result;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::events::game::RaidResult;
use crate::services::impls::combat_controller::ControllerMessage;
use crate::{
    configuration::ChannelDurations,
    events::game::{ChannelResult, ExploreResult, QuestResult},
    models::{quest::Quest, region::RegionName},
};

use super::{channel::ChannelingAction, explore::ExploreAction, off_beat_actions::OffBeatActions};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum ActionNames {
    Explore,
    Channel,
    Quest,
    Raid,
    Unique(OffBeatActions),
}

impl ActionNames {
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
            _ => {
                panic!("Invalid string for ActionNames: {}", action_name);
            }
        }
    }
}

impl TryFrom<String> for ActionNames {
    type Error = String; // Or define a more specific error type

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match ActionNames::from_str(&value) {
            Some(action_name) => Ok(action_name),
            None => Err(format!("Invalid string for ActionNames: {}", value)),
        }
    }
}

pub type Responder<T> = oneshot::Sender<Result<T>>;
pub type CmdResponder<T> = oneshot::Sender<T>;

#[derive(Debug, Clone, Serialize)]
pub enum TaskAction {
    Explore(ExploreAction),
    Channel(ChannelingAction),
    QuestAction(String, String),
    QuestComplete(String, Quest),
    QuestAccepted(String, String),
}

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
        resp: CmdResponder<ResponderType>,
        combat_tx: Sender<ControllerMessage>,
    },
    QuestActionDone(String, String),
    QuestCompleted(String, Quest),
    QuestAccepted {
        hero_id: String,
        quest_id: String,
        resp: Responder<()>,
    },
}

#[derive(Debug)]
pub enum ResponderType {
    StringResponse(String),
    UnitResponse(()),
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
    Raid(RaidResult),
}

impl TaskLootBox {
    pub fn name(&self) -> String {
        match self {
            TaskLootBox::Region(_) => "explore-lootbox".to_string(),
            TaskLootBox::Channel(_) => "channel-lootbox".to_string(),
            TaskLootBox::Quest(_) => "quest-lootbox".to_string(),
            TaskLootBox::Raid(_) => "raid-lootbox".to_string(),
        }
    }
}
