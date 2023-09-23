use prisma_client_rust::chrono;
use uuid::Uuid;

use crate::events::game::TaskAction;
use crate::{events::game::GameEvent, infra::Infra, services::traits::async_task::Task};
use flume::{unbounded, Receiver, Sender};
use std::{collections::HashMap, future::Future, sync::Arc};
use std::{pin::Pin, sync::Mutex};
use tracing::info;

#[derive(Clone, Debug)]
pub struct TaskManager {
    // ... fields to manage tasks...
    tasks: Arc<Mutex<HashMap<Uuid, TaskAction>>>,
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
    pub fn schedule_action(&self, event: TaskAction) {
        let event_clone = event.clone();

        let (action, id): (Box<dyn Task>, Uuid) = match event.clone() {
            TaskAction::Channel(action) => (
                Box::new(action.clone()),
                Uuid::parse_str(&action.hero_id()).unwrap(),
            ),
            TaskAction::Explore(action) => (
                Box::new(action.clone()),
                Uuid::parse_str(&action.hero_id()).unwrap(),
            ),
            // ... other cases
        };
        match self.tasks.lock() {
            Ok(mut tasks) => {
                tasks.insert(id, event.clone());
            }
            Err(_) => {}
        }
        let tx = self.task_complete_sender.clone();
        tokio::spawn(async move {
            /*  Doing the actual action here
             *
             *
             *  */
            let _ = action.execute().await;
            /* Signal the completion of the action here
             *
             *
             * */
            match &event_clone {
                TaskAction::Channel(channeling_action) => {
                    Infra::dispatch(GameEvent::ChannelingCompleted(channeling_action.clone()));
                }
                TaskAction::Explore(explore_action) => {
                    Infra::dispatch(GameEvent::ExploreCompleted(explore_action.clone()));
                }
                _ => {}
            }

            if let Err(err) = tx.send(id) {
                println!("Failed to send completion message: {:?}", err);
            }
        });
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

    pub fn get_task(&self, id: Uuid) -> Option<TaskAction> {
        match self.tasks.lock() {
            Ok(tasks) => tasks.get(&id).map(|task| task.clone()),
            Err(_) => None,
        }
    }

    pub fn get_current_task(&self, hero_id: &str) -> Option<TaskAction> {
        let tasks = match self.tasks.lock() {
            Ok(tasks) => tasks,
            Err(_) => return None,
        };

        tasks
            .values()
            .find(|task| matches!(task, TaskAction::Explore(t) if t.hero_id() == hero_id))
            .cloned()
    }
}
