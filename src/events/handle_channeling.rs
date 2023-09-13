use std::sync::Arc;

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
        Infra::subscribe(GameEvent::channeling_completed(), Arc::new(self.clone()));
    }
}

impl EventHandler for ChannelingHandler {
    fn handle(&self, event: GameEvent) {
        match event {
            GameEvent::Channeling(action) => {
                Infra::tasks().schedule_action(GameEvent::Channeling(action.clone()));
                println!("ChannelingHandler: {:?}", action);
            }
            GameEvent::ChannelingCompleted(action) => {
                println!("ChannelingHandler: {:?}", action);
            }
            _ => {}
        }
    }
}
