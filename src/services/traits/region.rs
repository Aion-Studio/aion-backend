use uuid::Uuid;

use crate::{
    models::region::{RegionActionResult, RegionName},
    services::tasks::explore::ExploreAction,
    types::RepoFuture,
};

use super::async_task::TaskError;

pub trait RegionService {
    fn start_exploration(
        &self,
        hero_id: String,
        region_name: RegionName,
    ) -> Result<Uuid, TaskError>;

    fn generate_result_for_exploration<'a>(
        &'a self,
        explore_action: &'a ExploreAction,
    ) -> RepoFuture<'a, RegionActionResult>;
    fn calculate_boost_factor(&self, exploration: i32) -> f64;
    fn results_by_hero(&self, hero_id: String) -> RepoFuture<Vec<RegionActionResult>>;
}
