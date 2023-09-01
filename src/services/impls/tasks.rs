use prisma_client_rust::chrono;
use uuid::Uuid;

use crate::{
    models::task::TaskKind,
    services::traits::{
        async_task::{Task, TaskError},
        scheduler::TaskScheduleResult,
    },
};
use flume::{unbounded, Receiver, Sender};
use std::{collections::HashMap, future::Future, sync::Arc};
use std::{pin::Pin, sync::Mutex};
use tracing::info;

#[derive(Clone, Debug)]
pub struct TaskManager {
    // ... fields to manage tasks...
    tasks: Arc<Mutex<HashMap<Uuid, TaskKind>>>,
    task_complete_sender: Sender<Uuid>,
}

impl TaskManager {
    pub fn new() -> Self {
        let (task_complete_sender, rx) = unbounded(); //
        let service = Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            task_complete_sender,
        };

        let fut = service.update_cache_on_complete(rx); // create the future that listens for completions
        tokio::spawn(fut); // spawn the future onto the tokio runtime

        service
    }
    /// Schedules a task to be executed asynchronously.
    ///
    /// # Parameters
    ///
    /// * `task` - The task to schedule. This is an enum of different task kinds.
    /// * `tx_task` - A sender channel for sending the completed task result.
    ///
    /// # Returns
    ///
    /// A `TaskScheduleResult` indicating success and containing the task ID if successful,
    /// or an error if the task could not be scheduled.
    ///
    /// # Functionality
    ///
    /// Matches on the task kind, currently only `TaskKind::Exploration` is supported.
    /// Clones the task and stores it in the `tasks` map.
    /// Spawns a new async task to execute the task.
    /// Sends the task ID via `tx` when completed.
    /// Sends the completed task via `tx_task` when completed.
    /// Returns the task ID if successfully inserted, error otherwise.
    pub fn schedule(
        &self,
        task: TaskKind,
        task_done_sender: Sender<TaskKind>,
    ) -> TaskScheduleResult {
        match task {
            TaskKind::Exploration(explore_task) => {
                let id = Uuid::parse_str(&explore_task.hero_id()).unwrap();
                explore_task.set_start_time(chrono::Utc::now());
                let tx = self.task_complete_sender.clone(); // clone the transmitter
                let task_clone = TaskKind::Exploration(explore_task.clone());
                tokio::spawn(async move {
                    // this is the time based work that the tokio task waits for
                    let _ = explore_task.execute().await;
                    if let Err(err) = tx.send(id) {
                        println!("Failed to send completion message: {:?}", err);
                    }
                    let _ = task_done_sender.send(TaskKind::Exploration(explore_task.clone()));
                });
                match self.tasks.lock() {
                    Ok(mut tasks) => {
                        tasks.insert(id, task_clone);
                        Ok(id)
                    }
                    Err(_) => Err(TaskError::InsertFailed),
                }
            }
        }
    }

    pub fn update_cache_on_complete(
        &self,
        mut rx: Receiver<Uuid>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let tasks = Arc::clone(&self.tasks);

        Box::pin(async move {
            while let Ok(id) = rx.recv_async().await {
                tasks.lock().unwrap().remove(&id);
            }
            info!("Stopped listening for completions.");
        })
    }

    pub fn get_task(&self, id: Uuid) -> Option<TaskKind> {
        match self.tasks.lock() {
            Ok(tasks) => tasks.get(&id).map(|task| task.clone()),
            Err(_) => None,
        }
    }

    pub fn get_current_task(&self, hero_id: &str) -> Option<TaskKind> {
        let tasks = match self.tasks.lock() {
            Ok(tasks) => tasks,
            Err(_) => return None,
        };

        tasks
            .values()
            .find(|task| matches!(task, TaskKind::Exploration(t) if t.hero_id() == hero_id))
            .cloned()
    }
}
