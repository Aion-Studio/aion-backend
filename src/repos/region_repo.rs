use std::sync::Arc;

use actix_web::web::Data;
use prisma_client_rust::QueryError;

use crate::{
    models::{hero::Hero, region::RegionActionResult, resources::Resource},
    prisma::{
        hero,
        region_action_result::{self, resources},
        resource_value::{self, resource},
        PrismaClient, ResourceType,
    },
};

#[derive(Clone)]
pub struct RegionRepo {
    prisma: Arc<Data<PrismaClient>>,
}

impl RegionRepo {
    pub fn new(prisma: Arc<Data<PrismaClient>>) -> Self {
        Self { prisma }
    }

    pub async fn get_hero(&self, hero_id: &str) -> Result<Hero, QueryError> {
        let h = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero_id.to_string()))
            .with(hero::attributes::fetch())
            .with(hero::base_stats::fetch())
            .exec()
            .await?;
        let hero = h.unwrap();
        Ok(hero.into())
    }
    pub async fn store_result(&self, result: RegionActionResult) -> Result<(), QueryError> {
        let action_result = self
            .prisma
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

    pub async fn results_by_hero(
        &self,
        hero_id: String,
    ) -> Result<Vec<RegionActionResult>, QueryError> {
        let results = self
            .prisma
            .region_action_result()
            .find_many(vec![region_action_result::hero::is(vec![
                hero::id::equals(hero_id),
            ])])
            .exec()
            .await
            .unwrap();

        let region_action_results: Vec<RegionActionResult> =
            results.into_iter().map(RegionActionResult::from).collect();

        Ok(region_action_results)
    }
}

impl From<region_action_result::Data> for RegionActionResult {
    fn from(data: region_action_result::Data) -> Self {
        let mut resources = Vec::new();

        // Unwrap the resources, replacing with an empty vector if None
        let data_resources = data.resources.unwrap_or_else(Vec::new);

        for data_resource in data_resources.iter() {
            // Convert data_resource.resource to a String or &str
            let resource_str = data_resource.resource.to_string();

            let resource = match resource_str.as_str() {
                "Aion" => Resource::Aion(0),
                "Valor" => Resource::Valor(0),
                "NexusShard" => Resource::NexusShard(0),
                _ => continue,
            };
            resources.push(resource);
        }

        Self {
            hero_id: data.hero_id,
            resources,
            xp: data.xp,
            discovery_level_increase: data.discovery_level_increase,
        }
    }
}

impl From<resource_value::resource::Set> for ResourceType {
    fn from(set: resource_value::resource::Set) -> Self {
        set.0
    }
}

impl<'a> From<&'a Resource> for ResourceType {
    fn from(resource: &'a Resource) -> Self {
        match resource {
            Resource::Aion(_) => ResourceType::Aion,

            Resource::Valor(_) => ResourceType::Valor,
            Resource::NexusShard(_) => ResourceType::NexusShard,
            Resource::Oak(_) => todo!(),
            Resource::IronOre(_) => todo!(),
            Resource::Copper(_) => todo!(),
            Resource::Silk(_) => todo!(),
        }
    }
}
