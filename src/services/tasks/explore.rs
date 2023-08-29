use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use prisma_client_rust::chrono::{self, Duration};
use rand::Rng;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use tokio::time::sleep;
use uuid::Uuid;

use crate::{
    models::region::RegionName,
    services::traits::async_task::{Task, TaskExecReturn, TaskStatus},
};

//TODO: move to models
#[derive(Clone, Debug)]
pub struct ExploreAction {
    id: Uuid,
    pub hero_id: String,
    pub duration: Duration,
    pub region_name: RegionName,
    pub xp: i32,
    pub discovery_level: i32,
    pub start_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    // Other fields needed for the exploration task...
}

impl Serialize for ExploreAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ExploreAction", 2)?;

        state.serialize_field("region_name", &self.region_name)?;

        let start_time = self.start_time.lock().unwrap();
        let datetime_str = start_time.map_or_else(|| "".to_string(), |dt| dt.to_rfc3339());
        state.serialize_field("start_time", &datetime_str)?;

        state.end()
    }
}

impl ExploreAction {
    pub fn new(
        hero_id: String,
        region_name: RegionName,
        durations: &HashMap<RegionName, Duration>,
    ) -> Self {
        // Implement task creation logic...
        let duration = *durations.get(&region_name).unwrap_or(&Duration::minutes(1));
        Self {
            id: Uuid::new_v4(),
            duration,
            hero_id,
            start_time: Arc::new(Mutex::new(None)),

            region_name,
            discovery_level: rand::thread_rng().gen_range(1..5),
            // random number between 15 and 30
            xp: rand::thread_rng().gen_range(15..30),
        }
    }

    pub fn set_start_time(&self, start_time: chrono::DateTime<chrono::Utc>) {
        let mut lock = self.start_time.lock().unwrap();
        *lock = Some(start_time);
    }
}

impl Task for ExploreAction {
    fn execute(&self) -> TaskExecReturn {
        let duration = self.duration;
        // Create a new Tokio task
        Box::pin(async move {
            println!("Exploring for {} seconds...",duration.num_seconds());
            sleep(duration.to_std().unwrap()).await;
            println!("Exploration complete!");
            Ok(())
        })
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let locked_start_time = self.start_time.lock().unwrap();
        locked_start_time.clone()
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn check_status(&self) -> TaskStatus {
        let start_time = self.start_time.lock().unwrap();
        if self.duration > (chrono::Utc::now() - start_time.unwrap()) {
            TaskStatus::InProgress
        } else {
            TaskStatus::Completed
        }
    }

    fn hero_id(&self) -> String {
        self.hero_id.clone()
    }
}
