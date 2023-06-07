use std::{future::Future, pin::Pin};

use prisma_client_rust::chrono::{self, Duration};
use uuid::Uuid;

pub type TaskExecReturn = Pin<Box<dyn Future<Output = Result<(), TaskError>> + Send>>;

pub trait Task {
    fn execute(&self) -> TaskExecReturn;
    fn id(&self) -> Uuid;
    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>>;
    fn duration(&self) -> Duration;
    fn check_status(&self) -> TaskStatus;
    fn hero_id(&self) -> String;
}

pub enum TaskStatus {
    InProgress,
    Completed,
    Error(TaskError),
}

pub enum TaskError {
    ExecutionFailed,
    InsertFailed,
    RepoError, // Different types of errors that can occur in a task
               // This will need to be customized for your application
}
