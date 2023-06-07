use uuid::Uuid;

use crate::services::{
    tasks::task_kind::TaskKind,
    traits::{
        async_task::{Task, TaskError},
        scheduler::TaskScheduler,
    },
};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::task::JoinHandle;

pub struct TaskSchedulerService {
    // ... fields to manage tasks...
    tasks: Mutex<HashMap<Uuid, (TaskKind, JoinHandle<Result<(), TaskError>>)>>,
}

impl TaskScheduler for TaskSchedulerService {
    fn schedule(&self, task: TaskKind) -> Result<Uuid, TaskError> {
        match &task {
            TaskKind::Exploration(explore_task) => {
                let id = explore_task.id();
                let handle = tokio::spawn(explore_task.execute());
                match self.tasks.lock() {
                    Ok(mut tasks) => {
                        tasks.insert(id, (task, handle));
                        Ok(id)
                    }
                    Err(_) => Err(TaskError::InsertFailed),
                }
            }
        }
        // Ok(Uuid::new_v4())
        // Implement task scheduling logic...
    }

    fn get_task(&self, id: Uuid) -> Option<TaskKind> {
        match self.tasks.lock() {
            Ok(tasks) => tasks.get(&id).map(|(task, _)| task.clone()),
            Err(_) => None,
        }
    }
}
