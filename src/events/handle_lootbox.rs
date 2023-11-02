use std::{collections::HashMap, sync::Arc};

use crate::events::game::ActionCompleted;
use crate::services::traits::async_task::Task;
use crate::{
    infra::Infra,
    models::resources::Resource,
    services::tasks::{channel::ChannelingAction, explore::ExploreAction},
};
use anyhow::Result;
use prisma_client_rust::chrono;
use rand::Rng;
use tracing::{error, info};
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
                    if let Ok(Channel(result)) = action.generate_loot_box(None) {
                        let mut hero = Infra::repo().get_hero(action.hero.get_id()).await.unwrap();
                        hero.equip_loot(Channel(result.clone()));
                        if let Err(e) = Infra::repo()
                            .store_action_completed(ActionCompleted::new(
                                action.action_name(),
                                action.hero.get_id(),
                                Channel(result),
                            ))
                            .await
                        {
                            error!("Error storing action completed: {}", e);
                        }
                        // update hero in db
                        println!("hero with stats about to be updated {:?}", hero);
                        if let Err(e) = Infra::repo().update_hero(hero).await {
                            error!("Error updating hero: {}", e);
                        }
                    }
                }
                GameEvent::ExploreCompleted(action) => {
                    let hero_id = action.clone().hero.id.unwrap();
                    let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                    let hero_region = Infra::repo()
                        .get_current_hero_region(&hero.get_id())
                        .await
                        .unwrap();
                    // update hero region discovery level
                    let loot =
                        match action.generate_loot_box(Some(hero_region.discovery_level as f64)) {
                            Ok(loot_box) => loot_box,
                            Err(err) => {
                                error!("Error generating lootbox: {}", err);
                                return;
                            }
                        };

                    hero.equip_loot(loot.clone());

                    if let Err(e) = Infra::repo()
                        .update_hero_region_discovery_level(&hero_id, action.discovery_level)
                        .await
                    {
                        error!("Error updating hero region discovery level: {}", e);
                    }

                    // Store action completed
                    if let Err(e) = Infra::repo()
                        .store_action_completed(ActionCompleted::new(
                            action.action_name(),
                            action.hero_id(),
                            loot,
                        ))
                        .await
                    {
                        error!("Error storing action completed: {}", e);
                    }

                    info!(
                        "Stored action completed {:?} for {:?}",
                        hero.name,
                        action.action_name()
                    );
                }
                _ => {}
            }
        });
    }
}

pub trait GeneratesResources<T> {
    fn generate_resources(&self, arg: Option<T>) -> HashMap<Resource, i32>;
}

pub trait LootBoxGenerator<T> {
    fn generate_loot_box(&self, arg: Option<T>) -> Result<TaskLootBox>;
}

// Implement the trait for ExploreAction
impl GeneratesResources<f64> for ExploreAction {
    fn generate_resources(&self, discovery: Option<f64>) -> HashMap<Resource, i32> {
        let material_reward = self.get_material_reward(discovery.unwrap());
        let mut loot = HashMap::new();
        loot.insert(Resource::NexusShard, rand::thread_rng().gen_range(5..20));
        loot.insert(Resource::Material(material_reward), 1);
        loot
    }
}

impl LootBoxGenerator<f64> for ExploreAction {
    fn generate_loot_box(&self, region_discovery: Option<f64>) -> Result<TaskLootBox> {
        let hero = self.hero.clone();
        let hero_id = hero.id.as_ref().unwrap();
        let boost_factor = self.calculate_boost_factor(self.hero.attributes.exploration.clone());
        let discovery_increase = self.discovery_level as f64 * boost_factor;
        let result = ExploreResult {
            xp: (self.xp as f64 * boost_factor) as i32,
            hero_id: hero_id.clone(),
            resources: self.generate_resources(region_discovery.map(|d| d + discovery_increase)),
            discovery_level_increase: discovery_increase,
            created_time: None,
        };

        Ok(TaskLootBox::Region(result))
    }
}

impl GeneratesResources<()> for ChannelingAction {
    fn generate_resources(&self, _: Option<()>) -> HashMap<Resource, i32> {
        let mut res = HashMap::new();
        res.insert(Resource::Aion, 50);
        res
    }
}

impl LootBoxGenerator<()> for ChannelingAction {
    fn generate_loot_box(&self, _: Option<()>) -> Result<TaskLootBox> {
        Ok(Channel(ChannelResult {
            xp: rand::thread_rng().gen_range(4..25),
            hero_id: self.hero.get_id(),
            resources: self.generate_resources(None),
            // random number between 5 and 20
            stamina_gained: rand::thread_rng().gen_range(5..20),
            created_time: Some(chrono::offset::Utc::now()),
        }))
    }
}
