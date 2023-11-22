use std::{collections::HashMap, sync::Arc};

use futures::future::{join_all, try_join_all};
use prisma_client_rust::{chrono, Direction, QueryError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, warn};

use crate::events::game::{ActionNames, TaskLootBox};
use crate::models::hero::{Attributes, BaseStats};
use crate::models::quest::{Action, Quest};
use crate::prisma::{action, hero_actions, hero_quests, quest, resource_type, ResourceEnum};
use crate::{
    events::game::ActionCompleted,
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionName},
        resources::Resource,
    },
    prisma::{
        action_completed, attributes, base_stats, hero,
        hero_region::{self, current_location, hero_id},
        hero_resource, inventory, leyline,
        region::{self, adjacent_regions},
        PrismaClient,
    },
    types::RepoFuture,
};

#[derive(Clone, Debug)]
pub struct Repo {
    prisma: Arc<PrismaClient>,
}

impl Repo {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }

    pub async fn insert_hero(&self, new_hero: Hero) -> Result<Hero, QueryError> {
        // Use Prisma to create a new Hero in the database
        // Convert the resulting record into a Hero struct and return it
        // ...
        let base_inventory = self.prisma.inventory().create(vec![]).exec().await.unwrap();

        let base_stats = self
            .prisma
            .base_stats()
            .create(
                new_hero.base_stats.level,
                new_hero.base_stats.xp,
                new_hero.base_stats.damage.min,
                new_hero.base_stats.damage.max,
                new_hero.base_stats.hit_points,
                new_hero.base_stats.mana,
                new_hero.base_stats.armor,
                vec![],
            )
            .exec()
            .await
            .unwrap();

        let base_attributes = self
            .prisma
            .attributes()
            .create(
                new_hero.attributes.strength,
                new_hero.attributes.resilience,
                new_hero.attributes.agility,
                new_hero.attributes.intelligence,
                new_hero.attributes.exploration,
                new_hero.attributes.crafting,
                vec![],
            )
            .exec()
            .await
            .unwrap();

        let result = self
            .prisma
            .hero()
            .create(
                new_hero.aion_capacity,
                new_hero.aion_collected,
                base_stats::id::equals(base_stats.clone().id),
                attributes::id::equals(base_attributes.clone().id),
                inventory::id::equals(base_inventory.clone().id),
                vec![hero::name::set(new_hero.name)],
            )
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .exec()
            .await?;
        let hero: Hero = result.into();
        let region_name = RegionName::Dusane;
        self.prisma
            .hero_region()
            .create(
                0,
                hero::id::equals(hero.get_id()),
                region::name::equals(region_name.to_str()),
                vec![current_location::set(true)],
            )
            .exec()
            .await?;
        let hero = self.hero_by_id(hero.get_id()).await.unwrap();
        Ok(hero)
    }
    pub fn get_hero(&self, hero_id: String) -> RepoFuture<Hero> {
        Box::pin(async move {
            match self.hero_by_id(hero_id).await {
                Ok(hero) => {
                    let last_action = self.latest_action_completed(hero.get_id()).await;
                    match last_action {
                        Ok(action_result) => {
                            let mut hero = hero.clone();
                            match action_result {
                                Some(action) => {
                                    hero.regenerate_stamina(&action);
                                    self.prisma
                                        .action_completed()
                                        .update(
                                            action_completed::id::equals(action.id),
                                            //Update updated_at to now
                                            vec![action_completed::updated_at::set(
                                                chrono::Utc::now().into(),
                                            )],
                                        )
                                        .exec()
                                        .await
                                        .unwrap();
                                    self.update_hero(hero.clone()).await
                                }
                                None => Ok(hero),
                            }
                        }
                        Err(e) => {
                            eprintln!("Error getting last action: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error getting hero: {}", e);
                    Err(e)
                }
            }
        })
    }

    async fn hero_by_id(&self, hero_id: String) -> Result<Hero, QueryError> {
        let hero = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero_id.clone()))
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .with(hero::hero_region::fetch(vec![hero_id::equals(
                hero_id.clone(),
            )]))
            .with(hero::resources::fetch(vec![
                hero_resource::hero_id::equals(hero_id.clone()),
            ]))
            .exec()
            .await?;
        match hero {
            Some(hero) => Ok(hero.into()),
            None => Err(QueryError::Serialize(format!(
                "No hero found with id: {}",
                hero_id
            ))),
        }
    }

    pub async fn update_hero(&self, hero: Hero) -> Result<Hero, QueryError> {
        self.update_base_stats(&hero.base_stats).await?;
        self.update_attributes(&hero.attributes).await?;
        self.update_hero_resources(&hero.resources, String::from(&hero.get_id()))
            .await?;

        let updated_hero = self
            .prisma
            .hero()
            .update(
                hero::id::equals(hero.get_id()),
                vec![
                    hero::aion_capacity::set(hero.aion_capacity),
                    hero::aion_collected::set(hero.aion_collected),
                    hero::stamina::set(hero.stamina),
                    hero::stamina_max::set(hero.stamina_max),
                    hero::stamina_regen_rate::set(hero.stamina_regen_rate),
                    //update base_stats with hero.base_stats
                ],
            )
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .with(hero::resources::fetch(vec![
                hero_resource::hero_id::equals(hero.get_id()),
            ]))
            .exec()
            .await?;

        Ok(updated_hero.into())
    }

    pub async fn update_hero_resources(
        &self,
        resources: &HashMap<Resource, i32>,
        hero_id: String,
    ) -> Result<(), QueryError> {
        let resource_creation_tasks: Vec<_> = resources
            .iter()
            .map(|(resource, amount)| {
                let resource_enum = ResourceEnum::from(resource.clone());
                self.prisma
                    .hero_resource()
                    .create(
                        hero::id::equals(hero_id.clone()),
                        resource_type::r#type::equals(resource_enum),
                        *amount,
                        vec![],
                    )
                    .exec()
            })
            .collect();
        let res = join_all(resource_creation_tasks).await;
        // iterate through and check if all have no errors
        for result in res {
            match result {
                Ok(_) => {}
                Err(e) => {
                    error!("Error creating hero resource: {}", e);
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    async fn update_base_stats(&self, base_stats: &BaseStats) -> Result<(), QueryError> {
        self.prisma
            .base_stats()
            .update(
                base_stats::id::equals(base_stats.clone().id.unwrap()),
                base_stats_update_params(&base_stats),
            )
            .exec()
            .await?;
        Ok(())
    }

    async fn update_attributes(&self, attributes: &Attributes) -> Result<(), QueryError> {
        self.prisma
            .attributes()
            .update(
                attributes::id::equals(attributes.clone().id.unwrap()),
                attributes_update_params(&attributes),
            )
            .exec()
            .await?;
        Ok(())
    }

    pub async fn add_quest(&self, quest: Quest) -> Result<(), QueryError> {
        let actions = quest.actions.clone();
        let action_futures = actions.into_iter().map(|action| async move {
            let existing = self
                .prisma
                .action()
                .find_first(vec![
                    action::name::equals(action.name.clone()),
                    action::region_name::equals(action.region_name.to_str()),
                ])
                .exec()
                .await;

            match existing {
                Ok(Some(action)) => Ok(action.id), // Assuming the ID is a String
                Ok(None) => {
                    // Create the action if it doesn't exist
                    let new_action = self
                        .prisma
                        .action()
                        .create(
                            action.name,
                            region::name::equals(action.region_name.to_str()),
                            vec![action::description::set(action.description)],
                        ) // You need to adjust the parameters accordingly
                        .exec()
                        .await?;
                    Ok(new_action.id)
                }
                Err(e) => Err(e.into()), // You might need to convert the error type here
            }
        });
        let action_ids_fetch: Result<Vec<String>, QueryError> = try_join_all(action_futures).await;

        let ids = match action_ids_fetch {
            Ok(ids) => ids,
            Err(e) => {
                error!("Error getting action ids: {}", e);
                return Err(e);
            }
        };

        let set_params = Quest::set(&quest, ids);

        let _ = self
            .prisma
            .quest()
            .create(
                quest.title,
                region::name::equals(quest.region_name),
                quest.quest_number,
                set_params,
            )
            .exec()
            .await?;
        Ok(())
    }

    pub async fn get_quest_by_hero_id(&self, hero_id: String) -> Result<Quest, QueryError> {
        let hero_quest = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id),
                hero_quests::completed::equals(false),
            ])
            .exec()
            .await?;
        let quest_id = hero_quest.unwrap().quest_id;
        let quest = self
            .prisma
            .quest()
            .find_unique(quest::id::equals(quest_id))
            .exec()
            .await;

        Ok(quest.unwrap().unwrap().into())
    }

    pub async fn mark_quest_complete(
        &self,
        hero_id: String,
        quest_id: String,
    ) -> Result<(), QueryError> {
        let hq = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id.clone()),
                hero_quests::quest_id::equals(quest_id.clone()),
            ])
            .exec()
            .await?
            .unwrap();

        self.prisma
            .hero_quests()
            .update(
                hero_quests::id::equals(hq.id),
                vec![hero_quests::completed::set(true)],
            )
            .exec()
            .await?;

        Ok(())
    }

    pub async fn get_hero_actions_by_hero_id(
        &self,
        hero_id: String,
    ) -> Result<Vec<String>, QueryError> {
        let hero_actions = self
            .prisma
            .hero_actions()
            .find_many(vec![hero_actions::hero_id::equals(hero_id)])
            .select(hero_actions::select!({ action_id }))
            .exec()
            .await?;
        Ok(hero_actions
            .into_iter()
            .map(|action| action.action_id)
            .collect())
    }

    pub async fn get_question_action_ids(
        &self,
        quest: Option<Quest>,
    ) -> Result<Vec<String>, QueryError> {
        let quest_id = quest.unwrap().id.unwrap();
        let actions = self
            .prisma
            .action()
            .find_many(vec![action::quest_id::equals(Some(quest_id))])
            .select(action::select!({ id }))
            .exec()
            .await?;
        Ok(actions.into_iter().map(|action| action.id).collect())
    }

    pub async fn get_available_quest(&self, hero_id: String) -> Result<Quest, QueryError> {
        let hero_quest_data = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id.clone()),
                hero_quests::completed::equals(false),
            ])
            .with(
                hero_quests::quest::fetch()
                    .with(quest::actions::fetch(vec![]).with(action::quest::fetch())),
            )
            .exec()
            .await?;

        let quest: Option<Quest> = hero_quest_data.and_then(|hero_quest| {
            hero_quest.quest.map(|boxed_quest_data| {
                // Dereference the boxed_quest_data and convert it to Quest
                (*boxed_quest_data).into()
            })
        });

        if let Some(quest) = quest {
            info!(
                "------------------Returning existing quest for hero: {}",
                hero_id
            );
            return Ok(quest);
        }

        let current_region = self.get_current_hero_region(&hero_id).await?;
        let first_quest_in_region = self
            .prisma
            .quest()
            .find_first(vec![
                quest::region_name::equals(current_region.region_name.to_str()),
                quest::quest_number::equals(1),
            ])
            .exec()
            .await?
            .unwrap();

        let hero_quest =
            self.prisma
                .hero_quests()
                .create(
                    hero::id::equals(hero_id.clone()),
                    quest::id::equals(first_quest_in_region.id),
                    vec![],
                )
                .with(
                    hero_quests::quest::fetch() //TODO: check if this works when more than one action
                        //is on a quest and 1 gets completed
                        .with(
                            quest::actions::fetch(vec![]).with(action::hero_action::fetch(vec![])),
                        ),
                )
                .exec()
                .await?;
        Ok((*hero_quest.quest.unwrap()).into())
    }

    pub async fn get_action_by_id(&self, action_id: &str) -> Result<Action, QueryError> {
        let action = self
            .prisma
            .action()
            .find_unique(action::UniqueWhereParam::IdEquals(String::from(action_id)))
            .exec()
            .await?;

        Ok(action.unwrap().into())
    }

    pub async fn add_hero_action(
        &self,
        hero_id: String,
        action_id: String,
    ) -> Result<(), QueryError> {
        self.prisma
            .hero_actions()
            .create(
                hero::id::equals(hero_id),
                action::id::equals(action_id),
                vec![],
            )
            .exec()
            .await?;
        Ok(())
    }

    pub async fn is_action_completed(
        &self,
        hero_id: String,
        action_id: String,
    ) -> Result<bool, QueryError> {
        let hero_action = self
            .prisma
            .hero_actions()
            .find_first(vec![
                hero_actions::hero_id::equals(hero_id),
                hero_actions::action_id::equals(action_id.clone()),
            ])
            .exec()
            .await?;

        match hero_action {
            Some(_) => {
                info!("Hero action already completed");
                return Ok(true);
            }
            None => {
                info!("Hero action not completed");
                return Ok(false);
            }
        }
    }

    pub async fn store_action_completed(&self, result: ActionCompleted) -> Result<(), QueryError> {
        //TODO: check lootbox created time
        let loot_box = match result.loot_box {
            // actionName key to lb value
            Some(lb) => match lb {
                TaskLootBox::Region(result) => {
                    json!({
                        "actionName": "Explore",
                        "result": result
                    })
                }
                TaskLootBox::Channel(result) => {
                    json!({
                        "actionName": "Channel",
                        "result": result
                    })
                }
            },
            None => {
                info!("NONEEEEEEEE");
                json!({})
            }
        };

        info!("\n loot box should be set {:?} \n", loot_box);
        let now = chrono::Utc::now().into();

        self.prisma
            .action_completed()
            .create(
                result.action_name.to_string(),
                hero::id::equals(result.hero_id),
                vec![
                    action_completed::loot_box::set(loot_box),
                    action_completed::created_at::set(now),
                ],
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

    pub async fn latest_action_completed(
        &self,
        hero_id: String,
    ) -> Result<Option<ActionCompleted>, QueryError> {
        let result = self
            .prisma
            .action_completed()
            .find_many(vec![action_completed::hero_id::equals(hero_id.to_string())])
            .order_by(action_completed::created_at::order(Direction::Desc))
            .take(1)
            .exec()
            .await
            .unwrap();
        //return first item of vec
        Ok(match result.into_iter().next() {
            Some(r) => Some(r.into()),
            None => None,
        })
    }

    pub async fn completed_actions(
        &self,
        take: i64,
        skip: i64,
    ) -> Result<Vec<ActionCompleted>, QueryError> {
        let data: Result<Vec<action_completed::Data>, QueryError> = self
            .prisma
            .action_completed()
            .find_many(vec![])
            .order_by(action_completed::created_at::order(Direction::Desc))
            .take(take)
            .skip(skip)
            .with(
                action_completed::hero::fetch()
                    .with(hero::base_stats::fetch())
                    .with(hero::attributes::fetch())
                    .with(hero::inventory::fetch())
                    .with(hero::resources::fetch(vec![])),
            )
            .exec()
            .await;
        match data {
            Ok(data) => Ok(data.into_iter().map(ActionCompleted::from).collect()),
            Err(e) => {
                error!("Error getting completed actions: {}", e);
                Err(e)
            }
        }
    }

    pub async fn latest_action_of_type(
        &self,
        hero_id: String,
        action_type: ActionNames,
    ) -> Result<Option<ActionCompleted>, QueryError> {
        let result = self
            .prisma
            .action_completed()
            .find_many(vec![
                action_completed::hero_id::equals(hero_id.to_string()),
                action_completed::action_name::equals(action_type.to_string()),
            ])
            .order_by(action_completed::created_at::order(Direction::Desc))
            .take(1)
            .exec()
            .await;

        match result {
            Ok(result) => Ok(match result.into_iter().next() {
                Some(r) => Some(r.into()),
                None => None,
            }),
            Err(e) => Err(e),
        }
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
        println!("inserted hero region");

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
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Error updating hero region discovery level: {}", e);
                Err(e)
            }
        }
    }

    pub async fn leylines_by_discovery(&self, hero_id: &str) -> Result<Vec<Leyline>, QueryError> {
        let hero_region = self.get_current_hero_region(hero_id).await?;
        let region_name = hero_region.region_name.clone();

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

    pub async fn get_all_heroes(&self) -> Result<Vec<(Hero, hero_region::Data)>, QueryError> {
        let heroes = self
            .prisma
            .hero()
            .find_many(vec![])
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .with(hero::resources::fetch(vec![]))
            .with(hero::hero_region::fetch(vec![]))
            .exec()
            .await?;
        let result: Vec<(Hero, hero_region::Data)> = heroes
            .into_iter()
            .flat_map(|hero_data| {
                let hero = Hero::from(hero_data.clone());
                hero_data
                    .hero_region
                    .unwrap_or_default()
                    .into_iter()
                    .map(move |region| (hero.clone(), region))
            })
            .collect();

        Ok(result)
    }
}

fn attributes_update_params(
    attributes: &crate::models::hero::Attributes,
) -> Vec<attributes::SetParam> {
    vec![
        attributes::strength::set(attributes.strength),
        attributes::resilience::set(attributes.resilience),
        attributes::agility::set(attributes.agility),
        attributes::intelligence::set(attributes.intelligence),
        attributes::exploration::set(attributes.exploration),
        attributes::crafting::set(attributes.crafting),
    ]
}
fn base_stats_update_params(base_stats: &BaseStats) -> Vec<base_stats::SetParam> {
    vec![
        base_stats::level::set(base_stats.level),
        base_stats::xp::set(base_stats.xp),
        base_stats::damage_min::set(base_stats.damage.min),
        base_stats::damage_max::set(base_stats.damage.max),
        base_stats::hit_points::set(base_stats.hit_points),
        base_stats::mana::set(base_stats.mana),
        base_stats::armor::set(base_stats.armor),
    ]
}

impl From<hero_resource::Data> for (Resource, i32) {
    fn from(data: hero_resource::Data) -> Self {
        let amount = data.amount;
        (Resource::from(data), amount)
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

#[derive(Deserialize, Serialize)]
struct ReturnTypeActionCompleted {
    id: String,
    action_name: String,
    hero_id: String,
    updated_at: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
    loot_box: serde_json::Value,
}
