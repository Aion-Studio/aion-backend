use std::{collections::HashMap, sync::Arc};

use super::game::GameEvent;

pub trait EventHandler: Send + Sync
where
    Self: std::fmt::Debug,
{
    fn handle(&self, event: GameEvent);
}

#[derive(Clone, Debug)]
pub struct EventDispatcher {
    subscribers: HashMap<String, Vec<Arc<dyn EventHandler + Send + Sync>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        let subscribers = HashMap::new();
        Self { subscribers }
    }

    pub fn subscribe(&mut self, event_name: &str, handler: Arc<dyn EventHandler + Send + Sync>) {
        let handlers = self.subscribers.entry(event_name.to_string()).or_default();
        handlers.push(handler);
    }

    pub fn dispatch(&self, event: GameEvent) {
        if let Some(handlers) = self.subscribers.get(&event.name()) {
            for handler in handlers {
                let event_clone = event.clone();
                let handler_clone = handler.clone(); // Clone the handler
                tokio::spawn(async move {
                    handler_clone.handle(event_clone);
                });
            }
        }
    }
}
