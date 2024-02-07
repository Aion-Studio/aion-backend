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
            let hero_region = Infra::repo()
                .get_current_hero_region(&hero_id)
                .await
                .unwrap();
            let stamina_cost =
                ExploreAction::get_stamina_cost(&region_name, hero_region.discovery_level);

            let action = Infra::repo()
                .get_hero(hero_id)
                .await
                .map(|hero| ExploreAction::new(hero, hero_region, stamina_cost))
                .unwrap_or_else(|_| None);

            match action {
                Some(action) => {
                    println!("hero can explore...should work");
                    action.start_now();
                    Infra::tasks().schedule_action(TaskAction::Explore(action));
                    let _ = resp.send(Ok(()));
                }
                None => {
                    print!("some shit happened ...");
                    let _ = resp.send(Err(anyhow::Error::msg("Not enough stamina")));
                }
            }
        });
    }
}
