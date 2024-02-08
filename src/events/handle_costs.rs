use serde_json::json;
use tracing::{error, info};

use crate::{
    infra::Infra,
    logger::log,
    repos::helpers::update_hero_db,
    services::tasks::action_names::{ActionNames, TaskAction},
};

#[derive(Clone, Debug)]
pub struct CostHandler {}

impl CostHandler {
    pub fn deduct_action_costs(action: TaskAction) {
        match action {
            TaskAction::Explore(action) => {
                let mut hero = action.hero;
               
                hero.deduct_stamina(action.stamina_cost);
                info!("hero should now have {:?} stamina", hero.stamina);

                let hero_id = hero.get_id();
                log(json!({"name":ActionNames::Explore.to_string(),"hero_id": hero_id.clone()}));

                log(
                    json!({"name": "Cost", "hero_id": hero_id.clone(), "resource_used": {
                        "stamina": action.stamina_cost
                    }}),
                );

                tokio::spawn(async move {
                    update_hero_db(hero).await;
                });
            }

            TaskAction::QuestAccepted(hero_id, quest_id) => {
                tokio::spawn(async move {
                    info!("paying shards to do quest for hero {}", hero_id);
                    let mut hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                    let quest = Infra::repo()
                        .get_quest_by_id(quest_id.clone())
                        .await
                        .unwrap();
                    hero.deduct_shards(&quest.cost);

                    update_hero_db(hero).await;
                });
            }
            _ => {} // TaskAction::Quest { .. } => {
                    //     println!("deduct_action_costs: Quest");
                    // }
        }
    }
}
