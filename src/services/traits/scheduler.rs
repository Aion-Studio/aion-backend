use crate::services::tasks::task_kind::TaskKind;

use super::async_task::{Task, TaskError};
use uuid::Uuid;

pub trait TaskScheduler: Send + Sync {
    fn schedule(&self, task: TaskKind) -> Result<Uuid, TaskError>;
    fn get_task(&self, id: Uuid) -> Option<TaskKind>;
}
