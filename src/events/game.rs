use prisma_client_rust::chrono::{self, Duration, Local, Utc};
use std::collections::HashMap;
use tracing::info;
use tracing::log::warn;

use crate::infra::Infra;
use crate::models::hero::Hero;
use crate::models::region::Leyline;
use crate::models::resources::Resource;
use crate::prisma::action_completed;
use prisma_client_rust::chrono::{DateTime, FixedOffset};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionNames {
    Explore,
    Channel,
    Quest,
    Raid
}

impl ActionNames {
    pub fn to_string(&self) -> String {
        match self {
            ActionNames::Explore => "Explore".to_string(),
            ActionNames::Channel => "Channel".to_string(),
            ActionNames::Quest => "Quest".to_string(),
            ActionNames::Raid => "Raid".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn from_json_to_loot_box(value: serde_json::Value) -> Option<TaskLootBox> {
    let mut map = value.as_object()?.clone();

    info!("converting data to loot box {:?}", map);

    let action_name = map.get("actionName")?.as_str()?;
    match action_name {
        "Explore" => {
            let result = map.remove("result")?;
            let result: ExploreResult = match serde_json::from_value(result.clone()) {
                Ok(explore_result) => explore_result,
                Err(e) => {
                    println!("error deserializing explore result: {} \n {:?}", e, result);
                    return None;
                }
            };

            Some(TaskLootBox::Region(result))
        }
        "Channel" => {
            let result = map.remove("result")?;
            let result: ChannelResult = serde_json::from_value(result).ok()?;
            Some(TaskLootBox::Channel(result))
        }
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChannelResult {
    pub hero_id: String,
    pub resources: HashMap<Resource, i32>,
    pub xp: i32,
    pub created_time: Option<DateTime<Utc>>,
    pub stamina_gained: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExploreResult {
    pub hero_id: String,
    #[serde(
        serialize_with = "serialize_resource_map",
        deserialize_with = "deserialize_resource_map"
    )]
    pub resources: HashMap<Resource, i32>,
    pub xp: i32,
    pub discovery_level_increase: f64,
    pub created_time: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionCompleted {
    pub id: String,
    #[serde(rename = "actionName")]
    pub action_name: ActionNames,
    #[serde(rename = "heroId")]
    pub hero_id: String,
    #[serde(rename = "hero")]
    pub hero: Option<Hero>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "completedAt")]
    pub completed_at: DateTime<FixedOffset>,
    #[serde(rename = "lootBox")]
    pub loot_box: Option<TaskLootBox>, // This is the new field
}

fn serialize_resource_map<S>(map: &HashMap<Resource, i32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(map.len()))?;
    for (key, value) in map {
        let entry = ResourceMapEntry {
            resource: key.clone(),
            amount: *value,
        };
        seq.serialize_element(&entry)?;
    }
    seq.end()
}

fn deserialize_resource_map<'de, D>(deserializer: D) -> Result<HashMap<Resource, i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec = Vec::<ResourceMapEntry>::deserialize(deserializer)?;

    let mut map = HashMap::new();
    for entry in vec {
        map.insert(entry.resource.clone(), entry.amount.clone());
    }

    Ok(map)
}

#[derive(Serialize, Deserialize)]
struct ResourceMapEntry {
    resource: Resource,
    amount: i32,
}

impl ActionCompleted {
    pub fn new(action_name: ActionNames, hero_id: String, loot_box: TaskLootBox) -> Self {
        let date = chrono::offset::Utc::now().into();
        ActionCompleted {
            id: uuid::Uuid::new_v4().to_string(),
            action_name,
            hero_id,
            hero: None,
            updated_at: date,
            created_at: date,
            completed_at: date,
            loot_box: Some(loot_box),
        }
    }

    pub async fn is_available(&self, hero: &Hero) -> bool {
        let now = chrono::Utc::now();
        let timeout = hero.timeout_durations(&self.action_name).await;
        self.updated_at + timeout > now
    }

    pub fn time_before_available(
        action_created_at: DateTime<Local>,
        timeout_duration: Duration,
    ) -> Option<Duration> {
        let now = Local::now().with_timezone(&Local);
        let time_until = action_created_at + timeout_duration;
        println!(
            "compare times createdAt: {:?}  timeout: {:?}  now: {:?}  time_until: {:?}",
            action_created_at, timeout_duration, now, time_until
        );

        let difference = time_until - now;

        if difference > Duration::seconds(0) {
            Some(difference)
        } else {
            None
        }
    }

    // returns leylines available to channel or the time when the next channeling action is available
    pub async fn channeling_available(
        &self,
        hero: &Hero,
    ) -> (Vec<Leyline>, chrono::DateTime<chrono::Local>) {
        if self.action_name != ActionNames::Channel {
            warn!("Calling channeling_available on non-channel action");
            return (vec![], chrono::Utc::now().with_timezone(&Local));
        }
        let hero_id = hero.get_id();
        let leylines = Infra::repo().leylines_by_discovery(&hero_id).await.unwrap();
        let timeout = hero.timeout_durations(&self.action_name).await;

        let time_until_available =
            ActionCompleted::time_before_available(self.created_at.with_timezone(&Local), timeout);

        info!(
            "hero and timeout {:?} -- time until avail {:?}",
            hero.name, time_until_available
        );
        if let Some(time_until) = time_until_available {
            (
                leylines,
                (chrono::Utc::now().with_timezone(&Local) + time_until).into(),
            )
        } else {
            (leylines, chrono::Utc::now().with_timezone(&Local).into())
        }
    }
}

impl From<action_completed::Data> for ActionCompleted {
    fn from(action_completed: action_completed::Data) -> Self {
        ActionCompleted {
            id: action_completed.id,
            action_name: ActionNames::from_string(&action_completed.action_name),
            hero_id: action_completed.hero_id,
            hero: action_completed
                .hero
                .map(|hero_data_box| Hero::from(*hero_data_box)), // Convert the hero data
            updated_at: action_completed.updated_at,
            created_at: action_completed.created_at,
            completed_at: action_completed.completed_at,
            loot_box: from_json_to_loot_box(action_completed.loot_box),
        }
    }
}

pub struct ActionDurations {}

impl ActionDurations {
    pub fn timeouts(action_name: &ActionNames) -> Duration {
        match action_name {
            ActionNames::Explore => Duration::minutes(3),
            ActionNames::Channel => Duration::minutes(3),
            ActionNames::Quest => Duration::minutes(3),
            ActionNames::Raid => Duration::minutes(3),
        }
    }
}
