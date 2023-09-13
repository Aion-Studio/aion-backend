use std::sync::Arc;

use prisma_client_rust::QueryError;

use crate::{
    infra::Infra,
    services::{tasks::explore::ExploreAction, traits::async_task::TaskError},
};

use super::{
    dispatcher::EventHandler,
    game::{GameEvent, RegionActionResult, TaskAction, TaskLootBox},
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

    pub async fn generate_loot_box(&self, action: &TaskAction) -> Result<TaskLootBox, TaskError> {
        // Generate loot box here
        match action {
            TaskAction::Explore(action) => {
                let hero = &action.hero;
                let hero_id = hero.id.as_ref().unwrap();
                let boost_factor =
                    self.calculate_boost_factor(action.hero.attributes.exploration.clone());
                let result = RegionActionResult {
                    xp: (action.xp as f64 * boost_factor) as i32,
                    hero_id: hero_id.clone(),
                    resources: vec![],
                    discovery_level_increase: (action.discovery_level as f64 * boost_factor),
                    created_time: None,
                };

                Ok(TaskLootBox::Region(result))
            }
            TaskAction::Channel(_) => todo!(),
        }
    }

    async fn hero_region_update(
        &self,
        // task_loot_box: &TaskLootBox,
        action_result: &RegionActionResult,
        action: ExploreAction,
    ) -> Result<(), QueryError> {
        // let discovery_level_increase = match &task_loot_box {
        //     TaskLootBox::Region(result) => result.discovery_level_increase,
        // };
        let hero_id = action.clone().hero.id.unwrap();
        Infra::repo()
            .update_hero_region_discovery_level(
                &hero_id,
                action_result.discovery_level_increase.round() as i32,
            )
            .await?;
        Ok(())
    }

    pub fn calculate_boost_factor(&self, exploration: i32) -> f64 {
        if exploration <= 10 {
            1.0
        } else {
            // Apply an exponential function where base_value = 10, max_value = 40, and growth_factor = 0.03
            let base_value = 10.0;
            let max_value = 40.0;
            let growth_factor = 0.03;

            // Calculate boost factor
            let boost: f64 = 1.0
                + ((max_value - base_value)
                    * (1.0 - (-growth_factor * (exploration as f64 - base_value)).exp()))
                .min(0.40);

            boost
        }
    }
}

impl EventHandler for LootBoxHandler {
    fn handle(&self, event: GameEvent) {
        let handler = self.clone();
        tokio::spawn(async move {
            match event {
                GameEvent::ChannelingCompleted(action) => {
                    let loot = handler
                        .generate_loot_box(&TaskAction::Channel(action.clone()))
                        .await;
                    if let Ok(TaskLootBox::Channel(result)) = &loot {

                    }
                }
                GameEvent::ExploreCompleted(action) => {
                    let task_action = TaskAction::Explore(action.clone());
                    let hero_id = action.clone().hero.id.unwrap();

                    if let Err(e) = Infra::repo()
                        .deduct_stamina(&hero_id, action.stamina_cost)
                        .await
                    {
                        eprintln!("Error deducting stamina: {:?}", e);
                    }

                    let loot = handler.generate_loot_box(&task_action).await;
                    if let Ok(TaskLootBox::Region(result)) = &loot {
                        if let Err(e) = handler.hero_region_update(&result, action.clone()).await {
                            eprintln!("Error updating hero region: {:?}", e);
                            panic!("Error updating hero region: {:?}", e);
                        }
                        if let Err(err) = Infra::repo()
                            .store_region_action_result(result.clone())
                            .await
                        {
                            eprintln!("Error storing region action result: {}", err);
                        }

                        //TODO: update hero stats from loot box items
                    }
                }
                _ => {}
            }
        });
    }
}
