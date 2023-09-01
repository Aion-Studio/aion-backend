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
    models::{hero::Hero, region::RegionName},
    services::traits::async_task::{Task, TaskExecReturn, TaskStatus},
};

//TODO: move to models
#[derive(Clone, Debug)]
pub struct ExploreAction {
    id: Uuid,
    pub hero: Hero,
    pub duration: Duration,
    pub region_name: RegionName,
    pub xp: i32,
    pub discovery_level: i32,
    pub start_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    pub stamina_cost: i32,
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
        hero: Hero,
        region_name: RegionName,
        durations: &HashMap<RegionName, Duration>,
    ) -> Option<Self> {
        // Implement task creation logic...
        let duration = *durations.get(&region_name).unwrap_or(&Duration::minutes(1));
        let stamina_cost = get_stamina_cost(&region_name);

        if (hero.stamina - stamina_cost) < 0 {
            return None;
        }

        Some(Self {
            id: Uuid::new_v4(),
            duration,
            hero,
            start_time: Arc::new(Mutex::new(None)),
            region_name: region_name.clone(),
            discovery_level: rand::thread_rng().gen_range(1..5),
            // random number between 15 and 30
            xp: rand::thread_rng().gen_range(15..30),
            stamina_cost,
        })
    }

    pub fn set_start_time(&self, start_time: chrono::DateTime<chrono::Utc>) {
        let mut lock = self.start_time.lock().unwrap();
        *lock = Some(start_time);
    }
}
fn get_stamina_cost(region_name: &RegionName) -> i32 {
    match region_name {
        RegionName::Forest => rand::thread_rng().gen_range(10..20),
        RegionName::Yezer => rand::thread_rng().gen_range(20..30),
        RegionName::Buzna => rand::thread_rng().gen_range(15..37),
        RegionName::Dusane => rand::thread_rng().gen_range(5..20),
        RegionName::Lindon => rand::thread_rng().gen_range(10..25),
        RegionName::Emerlad => rand::thread_rng().gen_range(25..45),
        RegionName::Veladria => rand::thread_rng().gen_range(30..50),
    }
}
impl Task for ExploreAction {
    fn execute(&self) -> TaskExecReturn {
        let duration = self.duration;
        // Create a new Tokio task
        Box::pin(async move {
            println!("Exploring for {} seconds...", duration.num_seconds());
            sleep(duration.to_std().unwrap()).await;
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
        self.hero.id.clone().unwrap()
    }
}
