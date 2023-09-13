use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use prisma_client_rust::chrono::{self, Duration};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use uuid::Uuid;

use crate::{
    models::{hero::Hero, region::Leyline},
    services::traits::async_task::{BaseTask, Task, TaskExecReturn, TaskStatus},
};

#[derive(Debug, Clone)]
pub struct ChannelingAction {
    id: Uuid,
    base: BaseTask,
    pub hero: Hero,
    pub leyline: Leyline,
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
        state.end()
    }
}

//TODO:add leyline name to duration to hashmap of durations
impl ChannelingAction {
    pub fn new(hero: Hero, leyline: Leyline, durations: &HashMap<&str, Duration>) -> Option<Self> {
        let duration = *durations
            .get(leyline.name.as_str())
            .unwrap_or(&Duration::minutes(1));

        Some(Self {
            id: Uuid::new_v4(),
            base: BaseTask::new(duration, hero.clone()),
            hero,
            leyline,
            start_time: Arc::new(Mutex::new(None)),
        })
    }
}

impl Task for ChannelingAction {
    fn execute(&self) -> TaskExecReturn {
        self.base.execute()
    }

    fn check_status(&self) -> TaskStatus {
        self.base.check_status()
    }

    fn hero_id(&self) -> String {
        self.base.hero_id()
    }

    fn task_id(&self) -> Uuid {
        self.base.task_id()
    }
}
