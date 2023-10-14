use std::sync::{Arc, Mutex};

use prisma_client_rust::chrono::{self, DateTime, Duration, Local, Utc};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use uuid::Uuid;

use crate::configuration::ChannelDurations;
use crate::events::game::ActionNames;
use crate::{
    models::hero::Hero,
    services::traits::async_task::{BaseTask, Task, TaskExecReturn, TaskStatus},
};

#[derive(Debug, Clone)]
pub struct ChannelingAction {
    id: Uuid,
    base: BaseTask,
    pub hero: Hero,
    pub leyline: String,
    pub start_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl Serialize for ChannelingAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChannelingAction", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("hero", &self.hero)?;
        state.serialize_field("leyline", &self.leyline)?;
        let time_left = self.base.get_end_time().unwrap();
        let local_datetime: DateTime<Local> = time_left.with_timezone(&Local);
        state.serialize_field("endTime", &local_datetime)?;
        state.end()
    }
}

impl ChannelingAction {
    pub fn new(hero: Hero, leyline_name: &str, durations: &ChannelDurations) -> Option<Self> {
        let duration = *durations
            .0
            .get(leyline_name)
            .unwrap_or(&Duration::minutes(1));

        Some(Self {
            id: Uuid::new_v4(),
            base: BaseTask::new(duration, hero.clone()),
            hero,
            leyline: leyline_name.to_string(),
            start_time: Arc::new(Mutex::new(None)),
        })
    }

    pub fn action_name(&self) -> ActionNames {
        ActionNames::Channel
    }
}

impl Task for ChannelingAction {
    fn execute(&self) -> TaskExecReturn {
        self.base.execute()
    }

    fn name(&self) -> String {
        ActionNames::Channel.to_string()
    }

    fn check_status(&self) -> TaskStatus {
        self.base.check_status()
    }

    fn start_now(&self) {
        self.base.start_now()
    }

    fn start_time(&self) -> Option<DateTime<Utc>> {
        self.base.start_time()
    }

    fn hero_id(&self) -> String {
        self.base.hero_id()
    }

    fn task_id(&self) -> Uuid {
        self.base.task_id()
    }
}
