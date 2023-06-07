use std::sync::Arc;

use actix_web::web::Data;
use uuid::Uuid;

use crate::{
    models::region::{RegionActionResult, RegionName},
    prisma::PrismaClient,
    repos::region_repo::RegionRepo,
    services::{
        tasks::{explore::ExploreAction, task_kind::TaskKind},
        traits::{
            async_task::{Task, TaskError},
            region::RegionService,
            scheduler::TaskScheduler,
        },
    },
    types::RepoFuture,
};

pub struct RegionServiceImpl {
    scheduler: Arc<dyn TaskScheduler>,
    repo: RegionRepo, // other dependencies...
}

impl RegionServiceImpl {
    pub fn new(scheduler: Arc<dyn TaskScheduler>, prisma: Data<PrismaClient>) -> Self {
        let repo = RegionRepo::new(Arc::new(prisma.clone()));
        Self {
            scheduler,
            repo,
            // other dependencies...
        }
    }
}

impl RegionService for RegionServiceImpl {
    fn start_exploration(
        &self,
        hero_id: String,
        region_name: RegionName,
    ) -> Result<Uuid, TaskError> {
        // create an ExplorationTask
        let task = ExploreAction::new(hero_id, region_name);

        // wrap the task in a TaskKind
        let task_kind = TaskKind::Exploration(task);

        // schedule the task
        let task_id = match self.scheduler.schedule(task_kind) {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        // return the task ID for later retrieval
        Ok(task_id)
    }

    fn generate_result_for_exploration<'a>(
        &'a self,
        explore_action: &'a ExploreAction,
    ) -> RepoFuture<'a, RegionActionResult> {
        let repo = self.repo.clone();
        let hero_id = explore_action.hero_id().to_owned();

        Box::pin(async move {
            let hero = match repo.get_hero(&hero_id).await {
                Ok(hero) => hero,
                Err(err) => return Err(err.into()),
            };

            let boost_factor = self.calculate_boost_factor(hero.attributes.exploration.clone());

            let result = RegionActionResult {
                xp: (explore_action.xp as f64 * boost_factor) as i32,
                resources: vec![],
                discovery_level_increase: (explore_action.discovery_level as f64 * boost_factor),
            };

            match repo.clone().store_result(result.clone()).await {
                Ok(_) => Ok(result),
                Err(err) => Err(err.into()),
            }
        })
    }

    fn calculate_boost_factor(&self, exploration: i32) -> f64 {
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
