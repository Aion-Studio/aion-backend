use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use flume::Sender;

use crate::models::task::{RegionActionResult, TaskLootBox};
use crate::services::impls::tasks::TaskManager;
use crate::{
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionName},
    },
    prisma::PrismaClient,
    repos::region_repo::RegionRepo,
    types::RepoFuture,
};

pub struct RegionService {
    scheduler: Arc<TaskManager>,
    repo: RegionRepo,
    // other dependencies...
    result_sender: Sender<TaskLootBox>,
}

impl RegionService {
    pub fn new(
        scheduler: Arc<TaskManager>,
        prisma: Arc<PrismaClient>,
        result_sender: Sender<TaskLootBox>,
    ) -> Arc<Self> {
        let repo = RegionRepo::new(prisma);

        let service = Arc::new(Self {
            result_sender,
            scheduler,
            repo,
        });

        service
    }

    pub fn get_hero_regions(&self, hero_id: String) -> RepoFuture<Vec<HeroRegion>> {
        let repo = self.repo.clone();

        Box::pin(async move {
            match repo.get_hero_regions(&hero_id.to_string()).await {
                Ok(hero_regions) => Ok(hero_regions),
                Err(err) => Err(err.into()),
            }
        })
    }

    // pub fn visible_leylines(&self, hero_id: String, discovery_points: ) -> RepoFuture<Vec<Leyline>> {
    //     let repo = self.repo.clone();
    //
    //     Box::pin(async move {
    //        match self.repo.leylines_by_discovery() 
    //     })
    // }
    //
    pub fn get_hero_current_region(&self, hero_id: String) -> RepoFuture<HeroRegion> {
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

    pub fn insert_new_region(
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

    pub fn create_region_hero(&self, hero: &Hero) -> RepoFuture<HeroRegion> {
        let repo = self.repo.clone();

        let hero = hero.clone();
        Box::pin(async move {
            let hero_region = repo.create_hero_region(&hero).await?;
            Ok(hero_region)
        })
    }

    // // historical lookup
    // pub fn results_by_hero(&self, hero_id: String) -> RepoFuture<Vec<RegionActionResult>> {
    //     let repo = self.repo.clone();
    //
    //     Box::pin(async move {
    //         let results = match repo.clone().results_by_hero(hero_id).await {
    //             Ok(results) => results,
    //             Err(err) => return Err(err.into()),
    //         };
    //
    //         Ok(results)
    //     })
    // }
}
