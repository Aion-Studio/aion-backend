use tracing::{error, info};

use crate::{events::game::TaskAction, services::traits::async_task::Task};
use std::sync::Arc;

use crate::infra::Infra;

use super::{dispatcher::EventHandler, game::GameEvent};

#[derive(Clone, Debug)]
pub struct ExploreHandler {}

impl ExploreHandler {
    pub fn new() -> Self {
        let handler = Self {};
        handler.subscribe();
        handler
    }

    pub fn subscribe(&self) {
        Infra::subscribe(GameEvent::hero_explores(), Arc::new(self.clone()));
        Infra::subscribe(GameEvent::explore_completed(), Arc::new(self.clone()));
    }
}

impl EventHandler for ExploreHandler {
    fn handle(&self, event: GameEvent) {
        match event {
            GameEvent::HeroExplores(action) => {
                action.start_now();
                Infra::tasks().schedule_action(TaskAction::Explore(action));
            }
            GameEvent::ExploreCompleted(action) => {
                let mut hero = action.hero;
                hero.deduct_stamina(action.stamina_cost);
                tokio::spawn(async move {
                    info!(
                        "Deducting stamina for explore completed for hero {}",
                        hero.get_id()
                    );
                    if let Err(e) = Infra::repo().update_hero(hero).await {
                        error!("Error updating hero: {}", e);
                    }
                });
            }
            _ => {}
        }
    }
}
