use std::collections::HashMap;

use anyhow::Result;
use prisma_client_rust::chrono;
use prisma_client_rust::chrono::DateTime;
use rand::Rng;
use serde_json::json;
use tracing::{error, info};

use TaskLootBox::*;

use crate::{
    infra::Infra,
    models::resources::Resource,
    services::tasks::{channel::ChannelingAction, explore::ExploreAction},
};
use crate::events::game::{ActionCompleted, RaidResult};
use crate::logger::Logger;
use crate::models::quest::{Action, Quest};
use crate::repos::helpers::update_hero_db;
use crate::services::tasks::action_names::{ActionNames, TaskAction, TaskLootBox};
use crate::services::tasks::explore::round;
use crate::services::traits::async_task::Task;

use super::game::{ChannelResult, ExploreResult};
use super::game::QuestResult;

#[derive(Clone, Debug)]
pub struct LootBoxHandler {}

impl LootBoxHandler {
    pub async fn create_lootbox_quest_action(quest_action: TaskAction) {
        match quest_action {
            TaskAction::QuestAction(hero_id, action_id) => {
                let action = Infra::repo().get_action_by_id(&action_id).await.unwrap();
                let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                let lootbox = match action.generate_loot_box(Some(hero_id)) {
                    Ok(loot_box) => loot_box,
                    Err(err) => {
                        error!("Error generating lootbox: {}", err);
                        return;
                    }
                };
                hero.equip_loot(lootbox.clone());
                update_hero_db(hero.clone()).await;
                info!(
                    "equipped hero with LootBox from questAction {:?}",
                    lootbox.clone()
                );
            }
            _ => {}
        }
    }
    pub async fn create_lootbox_explore(task_action: TaskAction) {
        info!("Creating lootbox for explore action ...");
        let action = match task_action {
            TaskAction::Explore(action) => action,
            _ => return,
        };
        let hero_id = action.clone().hero.id.unwrap();
        let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();

        let hero_region = Infra::repo()
            .get_current_hero_region(&hero.get_id())
            .await
            .unwrap();
        // update hero region discovery level
        let loot = match action.generate_loot_box(Some(hero_region.discovery_level as f64)) {
            Ok(loot_box) => loot_box,
            Err(err) => {
                error!("Error generating lootbox: {}", err);
                return;
            }
        };

        hero.equip_loot(loot.clone());
        update_hero_db(hero.clone()).await;

        // update hero_region discovery level

        if let Region(result) = loot.clone() {
            if let Err(e) = Infra::repo()
                .update_hero_region_discovery_level(
                    &hero_id,
                    result.discovery_level_increase as f64,
                )
                .await
            {
                error!("Error updating hero region discovery level: {}", e);
            }
        }

        // Store action completed
        if let Err(e) = Infra::repo()
            .store_action_completed(ActionCompleted::new(
                action.action_name(),
                action.hero_id(),
                loot.clone(),
            ))
            .await
        {
            error!("Error storing action completed: {}", e);
        }

        info!("storing loot box on LOGS {:?}", loot.clone());
        if let TaskLootBox::Region(loot_box) = loot.clone() {
            Logger::log(
                json!({"name":loot.name(),"hero_id":loot_box.hero_id, "resources": loot_box.resources,"xp": loot_box.xp, "discovery_inc": loot_box.discovery_level_increase}),
            );
        }

        info!(
            "Stored action completed {:?} for {:?}",
            hero.name,
            action.action_name()
        );
    }

    pub fn create_lootbox_channel(action: ChannelingAction) {
        tokio::spawn(async move {
            if let Ok(Channel(result)) = action.generate_loot_box(None) {
                info!("Channeling completed, generating lootbox...");
                let mut hero = Infra::repo().get_hero(action.hero.get_id()).await.unwrap();
                hero.equip_loot(Channel(result.clone()));
                if let Err(e) = Infra::repo()
                    .store_action_completed(ActionCompleted::new(
                        action.action_name(),
                        action.hero.get_id(),
                        Channel(result.clone()),
                    ))
                    .await
                {
                    error!("Error storing action completed: {}", e);
                }
                // update hero in db
                if let Err(e) = Infra::repo().update_hero(hero).await {
                    error!("Error updating hero: {}", e);
                }

                Logger::log(json!({
                    "name": result.name(),
                    "hero_id": result.hero_id,
                    "resources": result.resources,
                    "xp": result.xp,
                    "stamina_gained": result.stamina_gained,

                }));
            }
        });
    }

    pub fn create_lootbox_quest_complete(hero_id: String, quest: Quest) {
        tokio::spawn(async move {
            info!(
                "Generating lootbox for quest completed for hero {}",
                hero_id,
            );
            if let Ok(TaskLootBox::Quest(result)) = quest.generate_loot_box(Some(hero_id.clone())) {
                let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                hero.equip_loot(TaskLootBox::Quest(result.clone()));
                if let Err(e) = Infra::repo()
                    .store_action_completed(ActionCompleted::new(
                        ActionNames::Quest,
                        hero_id.clone(),
                        TaskLootBox::Quest(result.clone()),
                    ))
                    .await
                {
                    error!("Error storing action completed: {}", e);
                }

                if let Err(e) = Infra::repo().update_hero(hero).await {
                    error!("Error updating hero: {}", e);
                }

                Logger::log(
                    json!({"name": result.name(),"hero_id": result.hero_id, "resources": result.resources, "quest_id":result.quest_id}),
                );
            }
        });
    }
}

impl Default for TaskLootBox {
    fn default() -> Self {
        TaskLootBox::new()
    }
}

impl TaskLootBox {
    pub fn new() -> Self {
        //return a RegionActionResult
        Region(ExploreResult {
            hero_id: "hero_id".to_string(),
            resources: HashMap::new(),
            xp: 0,
            discovery_level_increase: 0.0,
            created_time: None,
        })
    }
}

pub trait GeneratesResources<T> {
    fn generate_resources(&self, arg: Option<T>) -> HashMap<Resource, i32>;
}

pub trait LootBoxGenerator<T> {
    fn generate_loot_box(&self, arg: Option<T>) -> Result<TaskLootBox>;
}

impl LootBoxGenerator<String> for Quest {
    fn generate_loot_box(&self, hero_id: Option<String>) -> Result<TaskLootBox> {
        let loot_box = QuestResult {
            hero_id: hero_id.unwrap(),
            quest_id: self.id.as_ref().unwrap().clone(),
            created_time: None,
            resources: self.generate_resources(None),
        };
        Ok(TaskLootBox::Quest(loot_box))
    }
}

impl GeneratesResources<()> for Quest {
    fn generate_resources(&self, _: Option<()>) -> HashMap<Resource, i32> {
        let mut res = HashMap::new();
        res.insert(Resource::NexusOrb, rand::thread_rng().gen_range(5..20));
        res
    }
}

// Implement the trait for ExploreAction
impl GeneratesResources<f64> for ExploreAction {
    fn generate_resources(&self, _: Option<f64>) -> HashMap<Resource, i32> {
        let mut loot = HashMap::new();
        loot.insert(Resource::StormShard, rand::thread_rng().gen_range(5..20));
        loot
    }
}

impl LootBoxGenerator<f64> for ExploreAction {
    fn generate_loot_box(&self, region_discovery: Option<f64>) -> Result<TaskLootBox> {
        let hero = self.hero.clone();
        let hero_id = hero.id.as_ref().unwrap();
        let boost_factor = self.calculate_boost_factor(self.hero.attributes.exploration.clone());
        let discovery_increase = round(self.discovery_level as f64 * boost_factor, 2);
        info!(
            "discovery level of action {} and boost factor {} , final result {}",
            self.discovery_level, boost_factor, discovery_increase
        );
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
// a static constant variable that maps npc monster's level to amount of valor resource to give, lvl 1-10 of monster, each level gives 35% more valor than previous  starting with 8
const VALOR: [i32; 10] = [8, 11, 15, 20, 27, 36, 49, 66, 89, 120];
impl GeneratesResources<()> for Action {
    fn generate_resources(&self, _: Option<()>) -> HashMap<Resource, i32> {
        let npc_level = self.npc.as_ref().unwrap().level;
        if let ActionNames::Raid = self.name {
            let valor = VALOR[npc_level as usize - 1];
            let mut res = HashMap::new();
            res.insert(Resource::Valor, valor);
            return res;
        }
        let mut res = HashMap::new();
        res
    }
}

const XP_GIVEN_BY_NPC: [i32; 10] = [10, 15, 20, 30, 45, 65, 90, 120, 155, 200];
impl LootBoxGenerator<String> for Action {
    fn generate_loot_box(&self, hero_id: Option<String>) -> Result<TaskLootBox> {
        let resources = self.generate_resources(None);
        let level = self.npc.as_ref().unwrap().level as usize - 1;
        let id = self.id.as_ref().unwrap().clone();
        Ok(Raid(RaidResult {
            hero_id: hero_id.unwrap(),
            xp: XP_GIVEN_BY_NPC[level],
            resources,
            created_time: Some(DateTime::from(chrono::offset::Utc::now())),
            action_id: id,
        }))
        // Ok()
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

pub fn from_json_to_loot_box(value: serde_json::Value) -> Option<TaskLootBox> {
    let mut map = value.as_object()?.clone();

    let action_name = map.get("actionName")?.as_str()?;
    match action_name {
        "Explore" => {
            let result = map.remove("result")?;
            let result: ExploreResult = match serde_json::from_value(result.clone()) {
                Ok(explore_result) => explore_result,
                Err(e) => {
                    error!("error deserializing explore result: {} \n {:?}", e, result);
                    return None;
                }
            };

            Some(TaskLootBox::Region(result))
        }
        "Channel" => {
            let result = map.remove("result")?;
            let result: ChannelResult = serde_json::from_value(result).ok()?;
            Some(TaskLootBox::Channel(result))
        }
        "Quest" => {
            let result = map.remove("result")?;
            let result: QuestResult = serde_json::from_value(result).ok()?;
            Some(TaskLootBox::Quest(result))
        }
        _ => None,
    }
}
