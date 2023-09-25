use std::{collections::HashMap, sync::Arc};

use crate::events::game::ActionCompleted;
use crate::services::traits::async_task::Task;
use crate::{
    infra::Infra,
    models::resources::Resource,
    services::tasks::{channel::ChannelingAction, explore::ExploreAction},
};
use anyhow::Result;
use rand::Rng;
use tracing::error;
use TaskLootBox::Channel;

use super::{
    dispatcher::EventHandler,
    game::{ChannelResult, ExploreResult, GameEvent, TaskLootBox},
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
        Infra::subscribe(GameEvent::channeling_completed(), Arc::new(self.clone()));
    }
}

impl EventHandler for LootBoxHandler {
    fn handle(&self, event: GameEvent) {
        tokio::spawn(async move {
            match event {
                GameEvent::ChannelingCompleted(action) => {
                    if let Ok(Channel(result)) = action.generate_loot_box() {
                        let mut hero = Infra::repo().get_hero(action.hero.get_id()).await.unwrap();
                        hero.equip_loot(Channel(result));
                        if let Err(e) = Infra::repo()
                            .store_action_completed(ActionCompleted::new(
                                action.name(),
                                action.hero.get_id(),
                            ))
                            .await
                        {
                            error!("Error storing action completed: {}", e);
                            eprintln!("Error storing action completed: {}", e);
                        }
                        // update hero in db
                        println!("hero with stats about to be updated {:?}", hero);
                        if let Err(e) = Infra::repo().update_hero(hero).await {
                            eprintln!("Error updating hero: {}", e);
                            error!("Error updating hero: {}", e);
                        }
                        println!("db updated for hero");
                    }
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
        Ok(Channel(ChannelResult {
            xp: rand::thread_rng().gen_range(4..25),
            hero_id: self.hero.get_id(),
            resources: self.generate_resources(),
            // random number between 5 and 20
            stamina_gained: rand::thread_rng().gen_range(5..20),
            created_time: None,
        }))
    }
}
