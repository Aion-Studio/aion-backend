use serde_json::json;
use tracing::{error, info};

use crate::{
    infra::Infra,
    logger::log,
    services::tasks::action_names::{ActionNames, TaskAction},
};

#[derive(Clone, Debug)]
pub struct CostHandler {}

impl CostHandler {
    pub fn deduct_action_costs(action: TaskAction) {
        println!("deduct_action_costs");
        match action {
            TaskAction::Explore(action) => {
                let mut hero = action.hero;
                hero.deduct_stamina(action.stamina_cost);

                log(json!({"name":ActionNames::Explore.to_string(),"hero_id":hero.get_id()}));

                log(
                    json!({"name": "Cost", "hero_id": hero.get_id(), "resource_used": {
                        "stamina": action.stamina_cost
                    }}),
                );

                tokio::spawn(async move {
                    info!(
                        "Deducting stamina for explore completed for hero {}",
                        hero.get_id()
                    );
                    if let Err(e) = Infra::repo().update_hero(hero).await {
                        error!("Error updating hero: {}", e);
                    }
                });

                println!("deduct_action_costs: Explore");
            }

            TaskAction::QuestAccepted(hero_id, quest_id) => {
                tokio::spawn(async move {
                    info!("paying shards to do quest for hero {}", hero_id);
                    let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                    let quest = Infra::repo().get_quest_by_id(quest_id.clone()).await.unwrap();
                    hero.deduct_shards(&quest.cost);

                    if let Err(e) = Infra::repo().update_hero(hero).await {
                        error!("Error updating hero after paying shards for quest: {}", e);
                    }
                });
            }
            _ => {} // TaskAction::Quest { .. } => {
                    //     println!("deduct_action_costs: Quest");
                    // }
        }
    }
}
