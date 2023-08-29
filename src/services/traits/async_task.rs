use std::{future::Future, pin::Pin};
use thiserror::Error;

use prisma_client_rust::chrono::{self, Duration};
use uuid::Uuid;

pub type TaskExecReturn = Pin<Box<dyn Future<Output=Result<(), TaskError>> + Send>>;
pub type TaskReturn = Result<Uuid, TaskError>;

pub trait Task {
    fn execute(&self) -> TaskExecReturn;
    fn id(&self) -> Uuid;
    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>>;
    fn duration(&self) -> Duration;
    fn check_status(&self) -> TaskStatus;
    fn hero_id(&self) -> String;
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
