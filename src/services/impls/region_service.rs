use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use actix_web::web::Data;
use prisma_client_rust::chrono::Duration;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionActionResult, RegionName, TaskResult},
        task::TaskKind,
    },
    prisma::PrismaClient,
    repos::region_repo::RegionRepo,
    services::{
        tasks::explore::ExploreAction,
        traits::{
            async_task::Task,
            region::RegionService,
            scheduler::{TaskScheduleResult, TaskScheduler},
        },
    },
    types::RepoFuture,
};

pub struct RegionServiceImpl {
    scheduler: Arc<dyn TaskScheduler>,
    repo: RegionRepo, // other dependencies...
    durations: HashMap<RegionName, Duration>,
    tx: Sender<TaskKind>,
}

impl RegionServiceImpl {
    pub fn new(
        scheduler: Arc<dyn TaskScheduler>,
        prisma: Data<PrismaClient>,
        durations: HashMap<RegionName, Duration>,
        result_tx: Sender<TaskResult>,
    ) -> Arc<Self> {
        let (tx, rx) = mpsc::channel(1200); // create a channel with a buffer size of 1200
        let repo = RegionRepo::new(Arc::new(prisma.clone()));

        let service = Arc::new(Self {
            tx,
            scheduler,
            repo,
            durations, // other dependencies...
        });

        let service_clone = Arc::clone(&service); // Clone the Arc
        let fut = service_clone.listen_for_explore_action(rx, result_tx); // create the future that listens for completions
        tokio::spawn(fut);

        service
    }
}

impl RegionService for RegionServiceImpl {
    fn start_exploration(&self, hero_id: String, region_name: RegionName) -> TaskScheduleResult {
        // create an ExplorationTask
        let task = ExploreAction::new(hero_id, region_name, &self.durations.clone());

        // wrap the task in a TaskKind
        let task_kind = TaskKind::Exploration(task);

        // schedule the task
        let task_id = match self.scheduler.schedule(task_kind, self.tx.clone()) {
            Ok(id) => id,
            Err(err) => return Err(err),
        };

        // return the task ID for later retrieval
        Ok(task_id)
    }

    fn listen_for_explore_action(
        self: Arc<Self>,
        mut rx: Receiver<TaskKind>,
        result_tx: Sender<TaskResult>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let this = self.clone();
        Box::pin(async move {
            while let Some(explore_action) = rx.recv().await {
                println!("Received explore action: {:?}", explore_action.clone());
                match explore_action {
                    TaskKind::Exploration(explore_action) => {
                        match this.generate_result_for_exploration(&explore_action).await {
                            Ok(res) => {
                                let _ = result_tx.send(TaskResult::Region(res)).await;
                            }
                            Err(_) => todo!(),
                        }

                        // if let Err(e) = this.generate_result_for_exploration(&explore_action).await
                        // {
                        //     println!("Error generating result for exploration: {}", e);
                        // }
                    }
                    TaskKind::Exploration(_) => todo!(),
                }
            }
        })
    }

    fn get_hero_regions(&self, hero_id: String) -> RepoFuture<Vec<HeroRegion>> {
        let repo = self.repo.clone();

        Box::pin(async move {
            match repo.get_hero_regions(&hero_id.to_string()).await {
                Ok(hero_regions) => Ok(hero_regions),
                Err(err) => Err(err.into()),
            }
        })
    }

    fn get_hero_current_region(&self, hero_id: String) -> RepoFuture<HeroRegion> {
        let repo = self.repo.clone();

        Box::pin(async move {
            match repo.get_hero_regions(&hero_id.to_string()).await {
                Ok(hero_region) => {
                    // find hero region with current_location true
                    let current_region = hero_region
                        .into_iter()
                        .find(|hr| hr.current_location == true)
                        .unwrap();
                    Ok(current_region)
                }
                Err(err) => Err(err.into()),
            }
        })
    }

    fn insert_new_region(
        &self,
        region_name: RegionName,
        adjacent_regions: Vec<String>,
    ) -> RepoFuture<Region> {
        let repo = self.repo.clone();

        Box::pin(async move {
            match repo.insert_new_region(region_name, adjacent_regions).await {
                Ok(region) => Ok(region),
                Err(err) => Err(err.into()),
            }
        })
    }

    fn insert_leyline(
        &self,
        region_name: RegionName,
        location: String,
        xp_reward: i32,
    ) -> RepoFuture<Leyline> {
        let repo = self.repo.clone();
        Box::pin(async move {
            match repo.add_leyline(region_name, location, xp_reward).await {
                Ok(leyline) => Ok(leyline),
                Err(err) => Err(err.into()),
            }
        })
    }

    fn create_region_hero(&self, hero: &Hero) -> RepoFuture<HeroRegion> {
        let repo = self.repo.clone();

        let hero = hero.clone();
        Box::pin(async move {
            let hero_region = repo.create_hero_region(&hero).await?;
            Ok(hero_region)
        })
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
                hero_id: hero.id.unwrap(),
                resources: vec![],
                discovery_level_increase: (explore_action.discovery_level as f64 * boost_factor),
            };
            match repo.clone().store_result(result.clone()).await {
                Ok(_) => Ok(result),
                Err(err) => Err(err.into()),
            }
        })
    }

    // historical lookup
    fn results_by_hero(&self, hero_id: String) -> RepoFuture<Vec<RegionActionResult>> {
        let repo = self.repo.clone();

        Box::pin(async move {
            let results = match repo.clone().results_by_hero(hero_id).await {
                Ok(results) => results,
                Err(err) => return Err(err.into()),
            };

            Ok(results)
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
