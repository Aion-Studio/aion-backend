use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};
use thiserror::Error;

use prisma_client_rust::chrono::{self, Duration};
use tokio::time::sleep;
use uuid::Uuid;

use crate::models::hero::Hero;

pub type TaskExecReturn = Pin<Box<dyn Future<Output = Result<(), TaskError>> + Send>>;
pub type TaskReturn = Result<Uuid, TaskError>;

pub trait Task: Send + Sync {
    fn execute(&self) -> TaskExecReturn;
    fn check_status(&self) -> TaskStatus;
    fn hero_id(&self) -> String;
    fn task_id(&self) -> Uuid;
}

#[derive(Clone, Debug)]
pub struct BaseTask {
    id: Uuid,
    hero: Hero,
    duration: Duration,
    start_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl BaseTask {
    pub fn new(duration: Duration, hero: Hero) -> Self {
        Self {
            id: Uuid::new_v4(),
            duration,
            hero,
            start_time: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_start_time(&self, start_time: chrono::DateTime<chrono::Utc>) {
        let mut lock = self.start_time.lock().unwrap();
        *lock = Some(start_time);
    }
}

impl Task for BaseTask {
    fn execute(&self) -> TaskExecReturn {
        let duration = self.duration;
        // Create a new Tokio task
        Box::pin(async move {
            println!(
                "Doing action for {} milliseconds...",
                duration.num_milliseconds()
            );
            sleep(duration.to_std().unwrap()).await;
            Ok(())
        })
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

    fn task_id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TaskStatus {
    InProgress,
    Completed,
    Error(TaskError),
    Failed,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TaskError {
    #[error("Execution failed")]
    ExecutionFailed,
    #[error("Insert failed")]
    InsertFailed,
    #[error("{0}")]
    RepoError(String), // Different types of errors that can occur in a task
}

impl From<prisma_client_rust::QueryError> for TaskError {
    fn from(e: prisma_client_rust::QueryError) -> Self {
        TaskError::RepoError(e.to_string())
    }
}
