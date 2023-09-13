use std::{future::Future, pin::Pin};


use crate::events::game::GameEvent;

use super::async_task::TaskError;
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

pub type TaskScheduleResult = Result<(Uuid), TaskError>;

pub trait TaskScheduler: Send + Sync {
    fn schedule(&self, task: GameEvent, tx: Sender<GameEvent>) -> TaskScheduleResult;
    fn get_task(&self, id: Uuid) -> Option<GameEvent>;
    fn get_current_task(&self, hero_id: &str) -> Option<GameEvent>;
    fn listen_for_completions(
        &self,
        rx: Receiver<Uuid>,
    ) -> Pin<Box<dyn Future<Output=()> + Send>>;
}
