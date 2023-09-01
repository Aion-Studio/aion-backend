use std::sync::Arc;

use prisma_client_rust::QueryError;

use crate::models::hero::Hero;
use crate::models::task::RegionActionResult;
use crate::prisma::hero::{self, stamina};
use crate::prisma::region_action_result::hero_id;
use crate::prisma::PrismaClient;

#[derive(Clone, Debug)]
pub struct GameEngineRepo {
    prisma: Arc<PrismaClient>,
}

impl GameEngineRepo {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }

    pub async fn store_region_action_result(
        &self,
        result: RegionActionResult,
    ) -> Result<(), QueryError> {
        self.prisma
            .region_action_result()
            .create(
                result.xp,
                result.discovery_level_increase,
                hero::id::equals(result.hero_id),
                // vec resu lt.resources
                vec![],
            )
            .exec()
            .await
            .unwrap(); // Implement result storage logic...

        Ok(())
    }
    pub async fn get_hero(&self, hero_id: String) -> Result<Hero, QueryError> {
        let hero = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero_id))
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .exec()
            .await?;
        Ok(hero.unwrap().into())
    }

    pub async fn deduct_stamina(&self, hero_id: &str, stamina: i32) -> Result<(), QueryError> {
        let hero = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero_id.clone().to_string()))
            .exec()
            .await;

        let hero = hero.unwrap();
        let new_stamina = match hero {
            Some(h) => h.stamina - stamina,
            None => 0,
        };

        self.prisma
            .hero()
            .update(
                hero::id::equals(hero_id.to_string()),
                vec![hero::stamina::set(new_stamina)],
            )
            .exec()
            .await?;
        Ok(())
    }

    // pub async fn action_results_by_hero(
    //     &self,
    //     hero_id: String,
    // ) -> Result<Vec<RegionActionResult>, QueryError> {
    //     let where_param = vec![hero_id::equals(hero_id.to_string())];
    //     let results = self
    //         .prisma
    //         .region_action_result()
    //         .find_many(where_param)
    //         .exec()
    //         .await
    //         .unwrap();
    //     Ok(results.into_iter().map(|r| r.into()).collect())
    // }
}
