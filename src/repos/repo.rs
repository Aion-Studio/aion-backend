use std::{collections::HashMap, sync::Arc};

use futures::future::{join_all, try_join_all};
use prisma_client_rust::{chrono, Direction, QueryError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{error, info, warn};

use crate::models::cards::{Card, Deck};
use crate::models::hero::{convert_to_fixed_offset, Attributes, BaseStats};
use crate::models::npc::Monster;
use crate::models::quest::{Action, HeroQuest, Quest};
use crate::prisma::{
    account, action, deck, deck_card, hero_actions, hero_quests, npc, npc_card, quest,
    resource_type, Resource,
};
use crate::repos::cards::CardRepo;
use crate::services::tasks::action_names::{ActionNames, TaskLootBox};
use crate::services::tasks::explore::ExploreAction;
use crate::utils::merge;
use crate::webserver::get_prisma_client;
use crate::{
    events::game::ActionCompleted,
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionName},
        resources::Resource,
    },
    prisma::{
        action_completed, hero,
        hero_region::{self, current_location, hero_id},
        hero_resource, leyline,
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
    pub fn new() -> Self {
        let prisma = get_prisma_client();
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
                new_hero.attributes.agility,
                new_hero.attributes.intelligence,
                new_hero.attributes.exploration,
                new_hero.attributes.crafting,
                vec![],
            )
            .exec()
            .await
            .unwrap();

        let account = self
            .prisma
            .account()
            .create("tempId123".to_string(), vec![])
            .exec()
            .await
            .unwrap();

        let result = self
            .prisma
            .hero()
            .create(
                new_hero.aion_capacity,
                base_stats::id::equals(base_stats.clone().id),
                attributes::id::equals(base_attributes.clone().id),
                inventory::id::equals(base_inventory.clone().id),
                account::id::equals(account.clone().id),
                vec![hero::name::set(new_hero.name)],
            )
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .exec()
            .await?;
        let hero: Hero = result.into();

        self.prisma
            .deck()
            .create(vec![deck::hero_id::set(Some(hero.get_id()))])
            .exec()
            .await?;
        let region_name = RegionName::Dusane;
        self.prisma
            .hero_region()
            .create(
                0.0,
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
                                    let _ = self.update_hero(hero.clone()).await;
                                    Ok(hero)
                                }
                                None => Ok(hero),
                            }
                        }
                        Err(e) => {
                            error!("Error getting last action: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error getting hero: {}", e);
                    Err(e)
                }
            }
        })
    }

    async fn hero_by_id(&self, hero_id: String) -> Result<Hero, QueryError> {
        let hero_data = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero_id.clone()))
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .with(hero::decks::fetch(vec![deck::hero_id::equals(Some(
                hero_id.clone(),
            ))]))
            .with(hero::hero_region::fetch(vec![hero_id::equals(
                hero_id.clone(),
            )]))
            .with(
                // fetch resources where the hero_resource id is our hero, then
                // include the resource for each hero_resource
                hero::resources::fetch(vec![hero_resource::hero_id::equals(hero_id.clone())])
                    .with(hero_resource::resource::fetch()),
            )
            .exec()
            .await?;

        let decks = hero_data.as_ref().unwrap().decks.clone().unwrap();

        let mut hero: Hero = match hero_data {
            Some(hero) => hero.into(),
            None => {
                return Err(QueryError::Serialize(format!(
                    "No hero found with id: {}",
                    hero_id
                )))
            }
        };

        let hero_id = hero.get_id();
        let all_decks_futures = decks
            .into_iter()
            .map(|deck| {
                let hero_id_ref = &hero_id;
                async move {
                    let cards: Vec<Card> = CardRepo::deck_cards_by_deck_id(deck.id.clone()).await;
                    Deck {
                        id: deck.id,
                        name: deck.name,
                        hero_id: Some(hero_id_ref.clone()),
                        cards_in_deck: cards,
                        active: deck.active,
                    }
                }
            })
            .collect::<Vec<_>>();
        let decks = join_all(all_decks_futures).await;
        hero.decks = Some(decks);

        Ok(hero)
        // hero
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
                    hero::stamina::set(hero.stamina),
                    hero::stamina_max::set(hero.stamina_max),
                    hero::stamina_regen_rate::set(hero.stamina_regen_rate),
                    hero::last_stamina_regeneration_time::set(convert_to_fixed_offset(
                        hero.last_stamina_regeneration_time,
                    )),
                    //update base_stats with hero.base_stats
                ],
            )
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .with(
                hero::resources::fetch(vec![hero_resource::hero_id::equals(hero.get_id())])
                    .with(hero_resource::resource::fetch()),
            )
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
                let prisma = self.prisma.clone();
                let hero_id_clone = hero_id.clone();

                // Use async block to await the outcome of find_first
                async move {
                    // Attempt to find an existing hero_resource
                    let hero_resource_id_result = prisma
                        .hero_resource()
                        .find_first(vec![
                            hero_resource::hero_id::equals(hero_id_clone.clone()),
                            hero_resource::resource::is(vec![resource_type::r#type::equals(
                                resource_enum.clone(),
                            )]),
                        ])
                        .exec()
                        .await;

                    match hero_resource_id_result {
                        Ok(Some(hero_resource)) => {
                            info!(
                                " hero {:?} resource {:?} and amount {:?} and wherecaluse",
                                hero_id_clone, resource_enum, amount,
                            );
                            prisma
                                .hero_resource()
                                .upsert(
                                    hero_resource::id::equals(hero_resource.id),
                                    hero_resource::create(
                                        hero::id::equals(hero_id_clone),
                                        resource_type::r#type::equals(resource_enum),
                                        *amount,
                                        vec![],
                                    ),
                                    vec![hero_resource::amount::set(*amount)],
                                )
                                .exec()
                                .await
                        }
                        _ => {
                            // If not found, or error, proceed to create a new hero_resource
                            prisma
                                .hero_resource()
                                .create(
                                    hero::id::equals(hero_id_clone),
                                    resource_type::r#type::equals(resource_enum),
                                    *amount,
                                    vec![],
                                )
                                .exec()
                                .await
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let res = join_all(resource_creation_tasks).await;
        // iterate through and check if all have no errors
        for result in res {
            info!("[repo] upsert result  {:?}", result);
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

    pub async fn get_quest_by_id(&self, quest_id: String) -> Result<Quest, QueryError> {
        let quest = self
            .prisma
            .quest()
            .find_unique(quest::id::equals(quest_id.clone()))
            .exec()
            .await?;
        match quest {
            Some(quest) => Ok(quest.into()),
            None => Err(QueryError::Serialize(format!(
                "No quest found with id: {}",
                quest_id
            ))),
        }
    }

    pub async fn add_quest(&self, quest: Quest) -> Result<(), QueryError> {
        let actions = quest.actions.clone();
        let action_futures = actions.into_iter().map(|action| async move {
            let existing = self
                .prisma
                .action()
                .find_first(vec![
                    action::name::equals(action.name.to_string().clone()),
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
                            action.name.to_string(),
                            region::name::equals(action.region_name.to_str()),
                            vec![action::description::set(action.description)],
                        )
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

    pub async fn get_hero_quest(
        &self,
        quest_id: String,
        hero_id: String,
    ) -> Result<HeroQuest, QueryError> {
        let hero_quests = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::quest_id::equals(quest_id.clone()),
                hero_quests::hero_id::equals(hero_id),
            ])
            .exec()
            .await?;
        match hero_quests {
            Some(hq) => Ok(hq.into()),
            None => Err(QueryError::Serialize(format!(
                "No hero quest found with id: {}",
                quest_id
            ))),
        }
    }

    pub async fn get_quest_by_hero_id(
        &self,
        hero_id: String,
    ) -> Result<(Quest, HeroQuest), QueryError> {
        let hero_quest = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id.clone()),
                hero_quests::completed::equals(false),
            ])
            .exec()
            .await?;

        // if hero has a quest but not completed, return it
        let hero_quest: HeroQuest = match hero_quest.clone() {
            Some(hq) => hq.into(),
            None => {
                let current_region = self.get_current_hero_region(&hero_id).await?;
                let first_quest_in_region = self
                    .prisma
                    .quest()
                    .find_first(vec![
                        quest::region_name::equals(current_region.region_name.to_str()),
                        quest::quest_number::equals(1),
                    ])
                    .exec()
                    .await?;

                let hero_quest = self
                    .prisma
                    .hero_quests()
                    .create(
                        hero::id::equals(hero_id.clone()),
                        quest::id::equals(first_quest_in_region.unwrap().id),
                        vec![],
                    )
                    .with(
                        hero_quests::quest::fetch() //TODO: check if this works when more than one action
                            //is on a quest and 1 gets completed
                            .with(
                                quest::actions::fetch(vec![])
                                    .with(action::hero_action::fetch(vec![])),
                            ),
                    )
                    .exec()
                    .await?;
                hero_quest.into()
            }
        };

        let quest_id = hero_quest.clone().quest_id;
        let quest = self
            .prisma
            .quest()
            .find_unique(quest::id::equals(quest_id))
            .exec()
            .await;

        Ok((quest.unwrap().unwrap().into(), hero_quest))
    }

    pub async fn mark_quest_complete(
        &self,
        hero_id: String,
        quest_id: &str,
    ) -> Result<(), QueryError> {
        let hq = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id.clone()),
                hero_quests::quest_id::equals(quest_id.to_string()),
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

    pub async fn get_quest_action_ids(&self, quest: Quest) -> Result<Vec<String>, QueryError> {
        let quest_id = quest.id.unwrap();
        let actions = self
            .prisma
            .action()
            .find_many(vec![action::quest_id::equals(Some(quest_id))])
            .select(action::select!({ id }))
            .exec()
            .await?;
        Ok(actions.into_iter().map(|action| action.id).collect())
    }

    pub async fn get_available_quest(
        &self,
        hero_id: String,
    ) -> Result<(Quest, HeroQuest), QueryError> {
        let hero_quest_data = self
            .prisma
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id.clone()),
                hero_quests::completed::equals(false),
            ])
            .with(
                hero_quests::quest::fetch().with(
                    quest::actions::fetch(vec![])
                        .with(action::quest::fetch())
                        // existence of action::hero_action means the hero completed the action
                        .with(action::hero_action::fetch(vec![
                            hero_actions::hero_id::equals(hero_id.clone()),
                        ])),
                ),
            )
            .exec()
            .await?;

        let hero_quest: Option<HeroQuest> = match hero_quest_data.clone() {
            Some(hq) => Some(hq.into()),
            None => None,
        };

        let quest: Option<Quest> = hero_quest_data.and_then(|hero_quest| {
            hero_quest.quest.map(|boxed_quest_data| {
                // Dereference the boxed_quest_data and convert it to Quest
                (*boxed_quest_data).into()
            })
        });

        if quest.is_some() && hero_quest.is_some() {
            return Ok((quest.unwrap(), hero_quest.unwrap()));
        }

        // if no quest is found, create a new one by creating hero_quest first

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
        Ok((
            (*hero_quest.clone().quest.unwrap()).into(),
            hero_quest.into(),
        ))
    }

    pub async fn accept_quest(&self, hero_id: String, quest_id: String) -> Result<(), QueryError> {
        self.prisma
            .hero_quests()
            .update_many(
                vec![
                    hero_quests::hero_id::equals(hero_id.clone()),
                    hero_quests::quest_id::equals(quest_id.clone()),
                ],
                vec![hero_quests::accepted::set(true)],
            )
            .exec()
            .await?;
        //now check
        // let is_accepted = self
        //     .prisma
        //     .hero_quests()
        //     .find_first(vec![
        //         hero_quests::hero_id::equals(hero_id.clone()),
        //         hero_quests::quest_id::equals(quest_id.clone()),
        //     ])
        //     .exec()
        //     .await?
        //     .unwrap()
        //     .accepted;
        Ok(())
    }

    pub async fn get_action_by_id(&self, action_id: &str) -> Result<Action, QueryError> {
        let action = self
            .prisma
            .action()
            .find_unique(action::UniqueWhereParam::IdEquals(String::from(action_id)))
            .with(action::quest::fetch())
            .with(action::npc::fetch().with(npc::deck::fetch()))
            .exec()
            .await?;

        Ok(action.unwrap().into())
    }

    /*
     Adds an action to hero's completed actions list
    */
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
                return Ok(true);
            }
            None => {
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
                TaskLootBox::Quest(result) => {
                    json!({
                        "actionName": "Quest",
                        "result": result
                    })
                }
                TaskLootBox::Raid(result) => {
                    json!({
                        "actionName": "Raid",
                        "result": result
                    })
                }
            },
            None => {
                json!({})
            }
        };

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
            .find_unique(hero::id::equals(hero_id.to_string()))
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
                0.0,
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
        discovery_level_increase: f64,
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
                leyline::discovery_required::lte(hero_region.discovery_level as i32),
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

    pub async fn get_or_create_hero_explore_action(
        &self,
        hero_id: &str,
    ) -> Result<Action, QueryError> {
        let hero_region = self.get_current_hero_region(hero_id).await?;
        let region_name = hero_region.region_name.clone();

        let actions = self
            .prisma
            .action()
            .find_many(vec![
                action::region_name::equals(region_name.to_str()),
                action::name::equals(ActionNames::Explore.to_string()),
            ])
            .exec()
            .await?;

        let action = actions.into_iter().next();

        match action {
            Some(action) => Ok(action.into()),
            None => {
                let new_action = self
                    .prisma
                    .action()
                    .create(
                        ActionNames::Explore.to_string(),
                        region::name::equals(region_name.to_str()),
                        vec![action::cost::set(Some(ExploreAction::get_stamina_cost(
                            &region_name,
                            hero_region.discovery_level,
                        )))],
                    )
                    .exec()
                    .await?;

                Ok(new_action.into())
            }
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

    pub async fn get_all_heroes(&self) -> Result<Vec<Value>, QueryError> {
        let heroes = self
            .prisma
            .hero()
            .find_many(vec![])
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .with(hero::resources::fetch(vec![]).with(hero_resource::resource::fetch()))
            .with(hero::hero_region::fetch(vec![]))
            .exec()
            .await?;
        let result: Result<Vec<Value>, serde_json::Error> = heroes
            .into_iter()
            .flat_map(|hero_data| {
                let hero = Hero::from(hero_data.clone());
                hero_data
                    .hero_region
                    .unwrap_or_default()
                    .into_iter()
                    .map(move |region| merge(hero.clone(), region))
            })
            .collect();

        result.map_err(|e| QueryError::Serialize(e.to_string()))
    }

    pub async fn get_npc_cards(&self, npc_id: &str) -> Result<Vec<Card>, QueryError> {
        let deck_cards = self
            .prisma
            .npc_card()
            .find_many(vec![npc_card::npc_id::equals(String::from(npc_id))])
            .with(npc_card::card::fetch())
            .exec()
            .await?;

        let cards_with_effects = deck_cards
            .into_iter()
            .map(|card| async move {
                let card_with_effects = CardRepo::fetch_card_with_effects(*(card.card.unwrap()))
                    .await
                    .unwrap();
                card_with_effects
            }) // Unwrap the card data
            .collect::<Vec<_>>();

        let cards = join_all(cards_with_effects).await;
        Ok(cards)
    }

    pub async fn get_npc_by_action_id(&self, action_id: &str) -> Result<Monster, QueryError> {
        let npc = self
            .prisma
            .npc()
            .find_first(vec![npc::WhereParam::ActionsSome(vec![
                action::id::equals(action_id.to_string()),
            ])])
            .with(npc::deck::fetch().with(deck::npc::fetch()))
            .exec()
            .await
            .unwrap()
            .unwrap();

        //we assume npc has a deck and cards when created
        let deck_cards = self.get_npc_cards(&npc.id).await.unwrap();

        let mut npc_obj: Monster = npc.into();
        npc_obj.deck = Some(Deck {
            id: "doesnt matter".to_string(),
            name: "npc deck".to_string(),
            active: true,
            hero_id: None,
            cards_in_deck: deck_cards,
        });
        Ok(npc_obj)
    }
}

fn attributes_update_params(attributes: &Attributes) -> Vec<attributes::SetParam> {
    vec![
        attributes::strength::set(attributes.strength),
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
        base_stats::resilience::set(base_stats.resilience),
        base_stats::damage_max::set(base_stats.damage.max),
        base_stats::hit_points::set(base_stats.hit_points),
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
