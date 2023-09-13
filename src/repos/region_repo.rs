use std::sync::Arc;

use prisma_client_rust::QueryError;
use tracing::{info, warn};

use crate::{
    events::game::{ChannelActionResult, RegionActionResult},
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionName},
        resources::Resource,
    },
    prisma::{
        hero,
        hero_region::{self, current_location, hero_id},
        leyline,
        region::{self, adjacent_regions},
        region_action_result, resource_value, PrismaClient, ResourceType,
    },
};

#[derive(Clone, Debug)]
pub struct Repo {
    prisma: Arc<PrismaClient>,
}

impl Repo {
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

    pub async fn consume_channelling_loot(
        &self,
        hero: &Hero,
        loot: &ChannelActionResult,
    ) -> Result<(), QueryError> {
        let hero_id = hero.id.as_ref().unwrap();
        let stamina = loot.stamina_gained;
        let xp = loot.xp;
        let resources = loot.resources.clone();

        self.deduct_stamina(hero_id, stamina).await?;
        self.prisma
            .hero()
            .update(
                hero::id::equals(hero_id.to_string()),
                vec![
                    hero::resources::increment(resources),
                ],
            )
            .exec()
            .await?;

        Ok(())
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

    pub async fn update_hero_region_discovery_level(
        &self,
        hero_id: &str,
        discovery_level_increase: i32,
    ) -> Result<(), QueryError> {
        let hero_region: HeroRegion = self.get_current_hero_region(hero_id).await?;

        let current_discovery = hero_region.discovery_level.clone();
        let set_params = HeroRegion::set(&HeroRegion {
            discovery_level: current_discovery + discovery_level_increase,
            ..hero_region.clone()
        });

        let result = self
            .prisma
            .hero_region()
            .update(hero_region::id::equals(hero_region.id.unwrap()), set_params)
            .exec()
            .await;

        match result {
            Ok(_) => {
                println!(
                    "updated hero region discover {:?}",
                    current_discovery + discovery_level_increase
                );
                Ok(())
            }
            Err(e) => {
                warn!("Error updating hero region discovery level: {}", e);
                Err(e)
            }
        }
    }

    pub async fn leylines_by_discovery(&self, hero_id: &str) -> Result<Vec<Leyline>, QueryError> {
        let hero_region = self.get_current_hero_region(hero_id).await?;
        let region_name = hero_region.region_name.clone();

        println!(
            "REPO:: hero region discovery_level {:?}",
            hero_region.discovery_level
        );

        // find leylines that have region_name as their region_name and discovery_required <= discovery_level
        let leylines = self
            .prisma
            .leyline()
            .find_many(vec![
                leyline::region_name::equals(region_name.to_str()),
                leyline::discovery_required::lte(hero_region.discovery_level),
            ])
            .exec()
            .await?;

        Ok(leylines.into_iter().map(Leyline::from).collect())
    }

    pub async fn get_hero_regions(&self, hero_id: &str) -> Result<Vec<HeroRegion>, QueryError> {
        let hero_region = self
            .prisma
            .hero_region()
            .find_many(vec![hero_id::equals(hero_id.to_string())])
            .with(hero_region::region::fetch())
            .exec()
            .await?;

        // maps the vec to the from impl
        Ok(hero_region.into_iter().map(HeroRegion::from).collect())
    }

    pub async fn get_current_hero_region(&self, hero_id: &str) -> Result<HeroRegion, QueryError> {
        let hero_region = self
            .prisma
            .hero_region()
            .find_first(vec![
                hero_id::equals(hero_id.to_string()),
                current_location::equals(true),
            ])
            .with(hero_region::region::fetch())
            .exec()
            .await;

        match hero_region {
            Ok(hero_region) => Ok(hero_region.unwrap().into()),
            Err(e) => Err(e),
        }
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

    // pub async fn add_leyline(
    //     &self,
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
            created_time: Some(data.create_time),
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
            id: Some(data.id),
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
            name: data.name,
            xp_reward: data.xp_reward,
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
            discovery_required: data.discovery_required,
            stamina_rate: data.stamina_rate,
            aion_rate: data.aion_rate,
        }
    }
}
