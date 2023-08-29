use std::sync::Arc;

use prisma_client_rust::QueryError;

use crate::prisma::{hero};
use crate::{prisma::PrismaClient};
use crate::models::task::RegionActionResult;

#[derive(Clone,Debug)]
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
                // vec result.resources
                vec![],
            )
            .exec()
            .await
            .unwrap(); // Implement result storage logic...

        Ok(())
    }
}
