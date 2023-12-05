use tracing::{error, info};

use crate::{
    models::region::RegionName,
    services::{
        tasks::{
            action_names::{Responder, TaskAction},
            explore::ExploreAction,
        },
        traits::async_task::Task,
    },
};

use crate::infra::Infra;

#[derive(Clone, Debug)]
pub struct ExploreHandler {}

impl ExploreHandler {
    pub fn hero_explores(hero_id: String, region_name: RegionName, resp: Responder<()>) {
        tokio::spawn(async move {
            let action = Infra::repo()
                .get_hero(hero_id)
                .await
                .map(|hero| ExploreAction::new(hero, region_name))
                .unwrap_or_else(|_| None);

            match action {
                Some(action) => {
                    action.start_now();
                    Infra::tasks().schedule_action(TaskAction::Explore(action));
                    let _ = resp.send(Ok(()));
                }
                None => {
                    let _ = resp.send(Err(anyhow::Error::msg("Not enough stamina")));
                }
            }
        });
    }

    pub fn explore_completed(action: ExploreAction) {
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
}
