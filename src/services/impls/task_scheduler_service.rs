use prisma_client_rust::chrono;
use uuid::Uuid;

use crate::{services::traits::{
    async_task::{Task, TaskError},
    scheduler::{TaskScheduleResult, TaskScheduler},
}, models::task::TaskKind};
use std::{collections::HashMap, future::Future, sync::Arc};
use std::{pin::Pin, sync::Mutex};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::{self, Receiver};

pub struct TaskSchedulerService {
    // ... fields to manage tasks...
    tasks: Arc<Mutex<HashMap<Uuid, TaskKind>>>,
    tx: Sender<Uuid>,
}

impl TaskSchedulerService {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(1200); // create a channel with a buffer size of 1200
        let service = Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            tx,
        };

        let fut = service.listen_for_completions(rx); // create the future that listens for completions
        tokio::spawn(fut); // spawn the future onto the tokio runtime

        service
    }
}

impl TaskScheduler for TaskSchedulerService {
    fn schedule(&self, task: TaskKind, tx_task: Sender<TaskKind>) -> TaskScheduleResult {
        match task {
            TaskKind::Exploration(explore_task) => {
                let id = explore_task.id();
                explore_task.set_start_time(chrono::Utc::now());
                let tx = self.tx.clone(); // clone the transmitter
                let task_clone = TaskKind::Exploration(explore_task.clone());
                tokio::spawn(async move {
                    let _ = explore_task.execute().await;
                    if let Err(err) = tx.send(id).await {
                        println!("Failed to send completion message: {:?}", err);
                    }

                    let _ = tx_task
                        .send(TaskKind::Exploration(explore_task.clone()))
                        .await;
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

    fn listen_for_completions(
        &self,
        mut rx: Receiver<Uuid>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let tasks = Arc::clone(&self.tasks);

        Box::pin(async move {
            println!("Starting to listen for completions.");
            while let Some(id) = rx.recv().await {
                println!("Received completion message for task {}", id);

                tasks.lock().unwrap().remove(&id);
            }
            println!("Stopped listening for completions.");
        })
    }

    fn get_task(&self, id: Uuid) -> Option<TaskKind> {
        match self.tasks.lock() {
            Ok(tasks) => tasks.get(&id).map(|task| task.clone()),
            Err(_) => None,
        }
    }

    fn get_current_task(&self, hero_id: &str) -> Option<TaskKind> {
        match self.tasks.lock() {
            Ok(tasks) => tasks
                .values()
                .find(|task| match task {
                    TaskKind::Exploration(explore_task) => explore_task.hero_id() == hero_id,
                })
                .map(|task| task.clone()),

            Err(_) => todo!(),
        }
    }
}
