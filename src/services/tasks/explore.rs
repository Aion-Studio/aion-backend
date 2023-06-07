use prisma_client_rust::chrono::{self, Duration};
use rand::Rng;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{
    models::region::RegionName,
    services::traits::async_task::{Task, TaskExecReturn, TaskStatus},
};

#[derive(Clone)]
pub struct ExploreAction {
    id: Uuid,
    pub hero_id: String,
    pub duration: Duration,
    pub region_name: RegionName,
    pub xp: i32,
    pub discovery_level: i32,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    // Other fields needed for the exploration task...
}

impl ExploreAction {
    pub fn new(hero_id: String, region_name: RegionName) -> Self {
        // Implement task creation logic...
        let duration = Duration::minutes(1);
        Self {
            id: Uuid::new_v4(),
            duration,
            hero_id,
            start_time: None,
            region_name,
            discovery_level: rand::thread_rng().gen_range(1..5),
            // random number between 15 and 30
            xp: rand::thread_rng().gen_range(15..30),
        }
    }
}

impl Task for ExploreAction {
    fn check_status(&self) -> TaskStatus {
        if self.duration > (chrono::Utc::now() - self.start_time.unwrap()) {
            TaskStatus::InProgress
        } else {
            TaskStatus::Completed
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn hero_id(&self) -> String {
        self.hero_id.clone()
    }

    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.start_time
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn execute(&self) -> TaskExecReturn {
        let duration = self.duration;
        // Create a new Tokio task
        Box::pin(async move {
            sleep(duration.to_std().unwrap()).await;
            Ok(())
        })
    }
}
