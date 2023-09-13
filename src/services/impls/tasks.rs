use prisma_client_rust::chrono;
use uuid::Uuid;

use crate::{
    events::game::GameEvent,
    infra::Infra,
    services::{
        tasks::{channel::ChannelingAction, explore::ExploreAction},
        traits::{
            async_task::{Task, TaskError},
            scheduler::TaskScheduleResult,
        },
    },
};
use flume::{unbounded, Receiver, Sender};
use std::{collections::HashMap, future::Future, sync::Arc};
use std::{pin::Pin, sync::Mutex};
use tracing::info;

#[derive(Clone, Debug)]
pub struct TaskManager {
    // ... fields to manage tasks...
    tasks: Arc<Mutex<HashMap<Uuid, GameEvent>>>,
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
    pub fn schedule(&self, task: GameEvent) -> TaskScheduleResult {
        match task {
            GameEvent::HeroExplores(explore_task) => {
                let id = Uuid::parse_str(&explore_task.hero_id()).unwrap();
                explore_task.set_start_time(chrono::Utc::now());
                let tx = self.task_complete_sender.clone(); // clone the transmitter
                let event_clone = GameEvent::HeroExplores(explore_task.clone());
                tokio::spawn(async move {
                    // this is the time based work that the tokio task waits for
                    let _ = explore_task.execute().await;
                    if let Err(err) = tx.send(id) {
                        println!("Failed to send completion message: {:?}", err);
                    }
                });
                match self.tasks.lock() {
                    Ok(mut tasks) => {
                        tasks.insert(id, event_clone);
                        Ok(id)
                    }
                    Err(_) => Err(TaskError::InsertFailed),
                }
            }
            GameEvent::ExploreCompleted(_) => todo!(),
            GameEvent::LootBoxCreated(_) => todo!(),
            GameEvent::Channeling(_) => todo!(),
            GameEvent::ChannelingCompleted(_) => todo!(),
        }
    }

    pub fn schedule_action(&self, event: GameEvent) {
        let event_clone = event.clone();

        let (action, id): (Box<dyn Task>, Uuid) = match event.clone() {
            GameEvent::Channeling(action) => (
                Box::new(action.clone()),
                Uuid::parse_str(&action.hero_id()).unwrap(),
            ),
            GameEvent::HeroExplores(action) => (
                Box::new(action.clone()),
                Uuid::parse_str(&action.hero_id()).unwrap(),
            ),
            GameEvent::ExploreCompleted(_) => todo!(),
            GameEvent::LootBoxCreated(_) => todo!(),
            GameEvent::ChannelingCompleted(_) => todo!(),
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
                GameEvent::Channeling(channeling_action) => {
                    Infra::dispatch(GameEvent::ChannelingCompleted(channeling_action.clone()));
                }
                GameEvent::HeroExplores(explore_action) => {
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

    pub fn get_task(&self, id: Uuid) -> Option<GameEvent> {
        match self.tasks.lock() {
            Ok(tasks) => tasks.get(&id).map(|task| task.clone()),
            Err(_) => None,
        }
    }

    pub fn get_current_task(&self, hero_id: &str) -> Option<GameEvent> {
        let tasks = match self.tasks.lock() {
            Ok(tasks) => tasks,
            Err(_) => return None,
        };

        tasks
            .values()
            .find(|task| matches!(task, GameEvent::HeroExplores(t) if t.hero_id() == hero_id))
            .cloned()
    }
}
