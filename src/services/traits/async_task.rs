use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};
use thiserror::Error;

use prisma_client_rust::chrono::{self, Duration, Local};
use tokio::time::sleep;
use tracing::error;
use uuid::Uuid;

use crate::models::hero::Hero;

pub type TaskExecReturn = Pin<Box<dyn Future<Output = Result<(), TaskError>> + Send>>;

pub trait Task: Send + Sync {
    fn execute(&self) -> TaskExecReturn;
    fn check_status(&self) -> TaskStatus;
    fn hero_id(&self) -> String;
    fn task_id(&self) -> Uuid;
    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>>;
    fn name(&self) -> String;
    fn start_now(&self);
}

pub trait GameAction<'a> {
    fn name(&self) -> String;
    fn hero_id(&self) -> String;
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

    // pub fn get_start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
    //     let start_time = self.start_time.lock().unwrap();
    //     start_time.clone()
    // }

    pub fn get_end_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let now = chrono::Utc::now();
        if let TaskStatus::Completed = self.check_status() {
            return Some(now);
        }
        let start_time = self.start_time.lock().unwrap();
        let start_time = start_time.clone();
        let time_left = start_time.unwrap() + self.duration;
        Some(time_left)
    }
}

impl Task for BaseTask {
    fn execute(&self) -> TaskExecReturn {
        let duration = self.duration;
        // Create a new Tokio task
        Box::pin(async move {
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

    fn start_now(&self) {
        let mut start_time = self.start_time.lock().unwrap();
        *start_time = Some(chrono::Utc::now());
        println!(
            "starting action now at {:?}",
            start_time.unwrap().with_timezone(&Local)
        );
    }

    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let start_time = self.start_time.lock().unwrap();
        start_time.clone()
    }

    fn hero_id(&self) -> String {
        self.hero.id.clone().unwrap()
    }

    fn task_id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> String {
        "base".to_string()
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    InProgress,
    Completed,
    // Error(TaskError),
    // Failed,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TaskError {
    // #[error("Execution failed")]
    // ExecutionFailed,
    // #[error("Insert failed")]
    // InsertFailed,
    #[error("{0}")]
    RepoError(String), // Different types of errors that can occur in a task
}

impl From<prisma_client_rust::QueryError> for TaskError {
    fn from(e: prisma_client_rust::QueryError) -> Self {
        TaskError::RepoError(e.to_string())
    }
}
