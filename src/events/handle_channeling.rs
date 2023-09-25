use crate::events::game::TaskAction;
use std::sync::Arc;
use tracing::info;

use crate::infra::Infra;

use super::{dispatcher::EventHandler, game::GameEvent};

#[derive(Debug, Clone)]
pub struct ChannelingHandler {}

impl ChannelingHandler {
    pub fn new() -> Self {
        let handler = Self {};
        handler.subscribe();
        handler
    }
    fn subscribe(&self) {
        Infra::subscribe(GameEvent::channeling(), Arc::new(self.clone()));
    }
}

impl EventHandler for ChannelingHandler {
    fn handle(&self, event: GameEvent) {
        match event {
            GameEvent::Channeling(action) => {
                Infra::tasks().schedule_action(TaskAction::Channel(action.clone()));
                info!("Scheduled channeling action for hero {:?}", action.hero.id);
            }
            GameEvent::ChannelingCompleted(_) => {
                info!("Channeling completed received inn channel handler");
            }
            _ => {}
        }
    }
}
