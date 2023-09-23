use std::{collections::HashMap, sync::Arc};

use crate::events::game::ActionCompleted;
use crate::services::traits::async_task::Task;
use crate::{
    infra::Infra,
    models::resources::Resource,
    services::{
        tasks::{channel::ChannelingAction, explore::ExploreAction},
        traits::async_task::TaskError,
    },
};
use anyhow::Result;
use prisma_client_rust::{chrono, QueryError};
use tracing::error;

use super::{
    dispatcher::EventHandler,
    game::{ChannelResult, ExploreResult, GameEvent, TaskAction, TaskLootBox},
};

#[derive(Clone, Debug)]
pub struct LootBoxHandler {}

impl LootBoxHandler {
    pub fn new() -> Self {
        let handler = Self {};
        handler.subscribe();
        handler
    }

    fn subscribe(&self) {
        Infra::subscribe(GameEvent::explore_completed(), Arc::new(self.clone()));
    }

    async fn hero_region_update(
        &self,
        action_result: &ExploreResult,
        action: ExploreAction,
    ) -> Result<(), QueryError> {
        let hero_id = action.clone().hero.id.unwrap();
        Infra::repo()
            .update_hero_region_discovery_level(
                &hero_id,
                action_result.discovery_level_increase.round() as i32,
            )
            .await?;
        Ok(())
    }
}

impl EventHandler for LootBoxHandler {
    fn handle(&self, event: GameEvent) {
        tokio::spawn(async move {
            match event {
                GameEvent::ChannelingCompleted(action) => {
                    let loot = action.generate_loot_box();
                }
                GameEvent::ExploreCompleted(action) => {
                    let hero_id = action.clone().hero.id.unwrap();
                    // update hero region discovery level
                    if let Err(e) = Infra::repo()
                        .update_hero_region_discovery_level(&hero_id, action.discovery_level)
                        .await
                    {
                        error!("Error updating hero region discovery level: {}", e);
                        eprintln!("Error updating hero region discovery level: {}", e);
                    }
                    // Store action completed
                    if let Err(e) = Infra::repo()
                        .store_action_completed(ActionCompleted::new(
                            action.name(),
                            action.hero_id(),
                        ))
                        .await
                    {
                        error!("Error storing action completed: {}", e);
                        eprintln!("Error storing action completed: {}", e);
                    }
                    let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                    // update hero stats and inventory
                    hero.deduct_stamina(action.stamina_cost);
                    let loot = action.generate_loot_box();
                    match loot {
                        Ok(loot_box) => hero.equip_loot(loot_box),
                        Err(err) => {
                            eprintln!("Error equipping loot box: {}", err);
                        }
                    }
                    // update hero in db
                    if let Err(e) = Infra::repo().update_hero(hero).await {
                        eprintln!("Error updating hero: {}", e);
                        error!("Error updating hero: {}", e);
                    }
                }
                _ => {}
            }
        });
    }
}

pub trait GeneratesResources {
    fn generate_resources(&self) -> HashMap<Resource, i32>;
}

pub trait LootBoxGenerator {
    fn generate_loot_box(&self) -> Result<TaskLootBox>;
}

// Implement the trait for ExploreAction
impl GeneratesResources for ExploreAction {
    fn generate_resources(&self) -> HashMap<Resource, i32> {
        HashMap::new()
        // Logic to generate resources for ExploreAction
    }
}

impl LootBoxGenerator for ExploreAction {
    fn generate_loot_box(&self) -> Result<TaskLootBox> {
        let hero = self.hero.clone();
        let hero_id = hero.id.as_ref().unwrap();
        let boost_factor = self.calculate_boost_factor(self.hero.attributes.exploration.clone());
        let result = ExploreResult {
            xp: (self.xp as f64 * boost_factor) as i32,
            hero_id: hero_id.clone(),
            resources: self.generate_resources(),
            discovery_level_increase: (self.discovery_level as f64 * boost_factor),
            created_time: None,
        };

        Ok(TaskLootBox::Region(result))
    }
}

impl GeneratesResources for ChannelingAction {
    fn generate_resources(&self) -> HashMap<Resource, i32> {
        let mut res = HashMap::new();
        res.insert(Resource::Aion, 50);
        res
    }
}

impl LootBoxGenerator for ChannelingAction {
    fn generate_loot_box(&self) -> Result<TaskLootBox> {
        Ok(TaskLootBox::Channel(ChannelResult {
            xp: 0,
            hero_id: self.hero.get_id(),
            resources: self.generate_resources(),
            stamina_gained: 15,
            //now
            created_time: None,
        }))
    }
}
