use std::sync::Arc;

use actix_web::web::Data;
use prisma_client_rust::QueryError;

use crate::{
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionActionResult, RegionName},
        resources::Resource,
    },
    prisma::{
        hero,
        hero_region::{self, current_location, hero_id},
        leyline,
        region::{self, adjacent_regions},
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

    pub async fn create_hero_region(&self, hero: &Hero) -> Result<HeroRegion, QueryError> {
        //Select a random enum variant from RegionName
        let region_name = RegionName::random();

        let hero_region = self
            .prisma
            .hero_region()
            .create(
                0,
                hero::id::equals(hero.get_id()),
                region::name::equals(region_name.to_str()),
                vec![current_location::set(true)],
            )
            .exec()
            .await?;

        Ok(hero_region.into())
    }

    pub async fn get_hero_regions(&self, hero_id: &str) -> Result<Vec<HeroRegion>, QueryError> {
        let hero_region = self
            .prisma
            .hero_region()
            //find first where hero id is equal to hero_id and current_location is true
            .find_many(vec![hero_id::equals(hero_id.to_string())])
            .with(hero_region::region::fetch())
            .exec()
            .await?;

        // maps the vec to the from impl
        Ok(hero_region.into_iter().map(HeroRegion::from).collect())
    }

    pub async fn insert_new_region(
        &self,
        region_name: RegionName,
        adjacent_regions: Vec<String>,
    ) -> Result<Region, QueryError> {
        let region = self
            .prisma
            .region()
            .create(
                region_name.to_str(),
                vec![adjacent_regions::set(adjacent_regions)],
            )
            .exec()
            .await?;

        Ok(region.into())
    }

    pub async fn add_leyline(
        &self,
        region_name: RegionName,
        location: String,
        xp_reward: i32,
    ) -> Result<Leyline, QueryError> {
        let leyline = self
            .prisma
            .leyline()
            .create(
                location,
                xp_reward,
                region::name::equals(region_name.to_str()),
                vec![],
            )
            .exec()
            .await?;

        Ok(leyline.into())
    }

    pub async fn store_result(&self, result: RegionActionResult) -> Result<(), QueryError> {
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

impl From<hero_region::Data> for HeroRegion {
    fn from(data: hero_region::Data) -> Self {
        Self {
            hero_id: data.hero_id,
            region_name: match data.region_name.as_str() {
                "Dusane" => RegionName::Dusane,
                "Yezer" => RegionName::Yezer,
                "Emerlad" => RegionName::Emerlad,
                "Forest" => RegionName::Forest,
                "Buzna" => RegionName::Buzna,
                "Veladria" => RegionName::Veladria,
                "Lindon" => RegionName::Lindon,
                _ => panic!("Unexpected region name"),
            },
            discovery_level: data.discovery_level,
            current_location: data.current_location,
        }
    }
}

impl From<region::Data> for Region {
    fn from(data: region::Data) -> Self {
        Self {
            name: match data.name.as_str() {
                "Dusane" => RegionName::Dusane,
                "Yezer" => RegionName::Yezer,
                "Emerlad" => RegionName::Emerlad,
                "Forest" => RegionName::Forest,
                "Buzna" => RegionName::Buzna,
                "Veladria" => RegionName::Veladria,
                "Lindon" => RegionName::Lindon,
                _ => panic!("Unexpected region name"),
            },
            adjacent_regions: data.adjacent_regions,
            leylines: data
                .leylines
                .unwrap_or_else(Vec::new)
                .into_iter()
                .map(|l| l.into())
                .collect(),
        }
    }
}

impl From<leyline::Data> for Leyline {
    fn from(data: leyline::Data) -> Self {
        Self {
            location: data.location,
            xp_reward: data.xp_reward,
        }
    }
}
