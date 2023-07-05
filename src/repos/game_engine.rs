use std::sync::Arc;

use actix_web::web::Data;
use prisma_client_rust::QueryError;

use crate::models::hero::{Attributes, BaseStats, Follower, Inventory, Item, Range, RetinueSlot};
use crate::models::region::RegionActionResult;
use crate::prisma::{attributes, base_stats, follower, hero, inventory, item, retinue_slot};
use crate::{models::hero::Hero, prisma::PrismaClient};

#[derive(Clone)]
pub struct GameEngineRepo {
    prisma: Arc<Data<PrismaClient>>,
}

impl GameEngineRepo {
    pub fn new(prisma: Arc<Data<PrismaClient>>) -> Self {
        Self { prisma }
    }

    pub async fn store_region_action_result(&self, result: RegionActionResult) -> Result<(), QueryError> {
        self.prisma
            .region_action_result()
            .create(
                result.xp,
                result.discovery_level_increase,
                hero::id::equals(result.hero_id),
                // vec result.resources
                vec![],
            )
            .exec()
            .await
            .unwrap(); // Implement result storage logic...

        Ok(())
    }
}
