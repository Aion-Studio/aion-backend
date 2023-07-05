use std::{future::Future, pin::Pin, sync::Arc};

use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::{
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionActionResult, RegionName, TaskResult}, task::TaskKind,
    },
    services::tasks::{explore::ExploreAction },
    types::{QueryResult, RepoFuture},
};

use super::{async_task::TaskError, scheduler::TaskScheduleResult};

pub trait RegionService {
    fn start_exploration(&self, hero_id: String, region_name: RegionName) -> TaskScheduleResult;
    fn get_hero_regions(&self, hero_id: String) -> RepoFuture<Vec<HeroRegion>>;
    fn generate_result_for_exploration<'a>(
        &'a self,
        explore_action: &'a ExploreAction,
    ) -> RepoFuture<'a, RegionActionResult>;
    fn calculate_boost_factor(&self, exploration: i32) -> f64;
    fn results_by_hero(&self, hero_id: String) -> RepoFuture<Vec<RegionActionResult>>;
    fn create_region_hero(&self, hero: &Hero) -> RepoFuture<HeroRegion>;
    fn get_hero_current_region(&self, hero_id: String) -> RepoFuture<HeroRegion>;
    fn listen_for_explore_action(
        self: Arc<Self>,
        rx: Receiver<TaskKind>,
        result_tx: tokio::sync::mpsc::Sender<TaskResult>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>;
    fn insert_new_region(
        &self,
        region_name: RegionName,
        adjacent_regions: Vec<String>,
    ) -> RepoFuture<Region>;

    fn insert_leyline(
        &self,
        region_name: RegionName,
        location: String,
        xp_reward: i32,
    ) -> RepoFuture<Leyline>;
}
