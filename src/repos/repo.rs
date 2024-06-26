use futures::future::try_join_all;
use prisma_client_rust::{chrono, Direction, QueryError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, warn};

use crate::db;
use crate::models::npc::Monster;
use crate::models::quest::{Action, HeroQuest, Quest};
use crate::models::resources::Resource;
use crate::prisma::{action, hero_actions, hero_quests, npc, quest};
use crate::services::tasks::action_names::{ActionNames, TaskLootBox};
use crate::services::tasks::explore::ExploreAction;
use crate::{
    events::game::ActionCompleted,
    models::{
        hero::Hero,
        region::{HeroRegion, Leyline, Region, RegionName},
    },
    prisma::{
        action_completed, hero,
        hero_region::{self, current_location, hero_id},
        hero_resource, leyline,
        region::{self, adjacent_regions},
    },
};

#[derive(Clone, Debug)]
pub struct Repo {}

impl Repo {
    pub async fn get_quest_by_id(&self, quest_id: String) -> Result<Quest, QueryError> {
        let quest = db!()
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
            let existing = db!()
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
                    let new_action = db!()
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

        let _ = db!()
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
        let hero_quests = db!()
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
        let hero_quest = db!()
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
                let first_quest_in_region = db!()
                    .quest()
                    .find_first(vec![
                        quest::region_name::equals(current_region.region_name.to_str()),
                        quest::quest_number::equals(1),
                    ])
                    .exec()
                    .await?;

                let first_quest_id = match first_quest_in_region {
                    Some(quest) => quest.id,
                    None => "none".to_string(),
                };

                let hero_quest = db!()
                    .hero_quests()
                    .create(
                        hero::id::equals(hero_id.clone()),
                        quest::id::equals(first_quest_id),
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
        let quest = db!()
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
        let hq = db!()
            .hero_quests()
            .find_first(vec![
                hero_quests::hero_id::equals(hero_id.clone()),
                hero_quests::quest_id::equals(quest_id.to_string()),
            ])
            .exec()
            .await?
            .unwrap();

        db!()
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
        let hero_actions = db!()
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
        let actions = db!()
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
        let hero_quest_data = db!()
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
        let first_quest_in_region = db!()
            .quest()
            .find_first(vec![
                quest::region_name::equals(current_region.region_name.to_str()),
                quest::quest_number::equals(1),
            ])
            .exec()
            .await?
            .unwrap();

        let hero_quest =
            db!()
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
        db!()
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
        let action = db!()
            .action()
            .find_unique(action::UniqueWhereParam::IdEquals(String::from(action_id)))
            .with(action::quest::fetch())
            .with(action::npc::fetch())
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
        db!()
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
        let hero_action = db!()
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

        db!()
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

    pub async fn latest_action_completed(
        &self,
        hero_id: String,
    ) -> Result<Option<ActionCompleted>, QueryError> {
        let result = db!()
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
        let data: Result<Vec<action_completed::Data>, QueryError> = db!()
            .action_completed()
            .find_many(vec![])
            .order_by(action_completed::created_at::order(Direction::Desc))
            .take(take)
            .skip(skip)
            .with(action_completed::hero::fetch())
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
        let result = db!()
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

        let hero_region = db!()
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

        let result = db!()
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
        let leylines = db!()
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
        let hero_region = db!()
            .hero_region()
            .find_many(vec![hero_id::equals(hero_id.to_string())])
            .with(hero_region::region::fetch())
            .exec()
            .await?;

        // maps the vec to the from impl
        Ok(hero_region.into_iter().map(HeroRegion::from).collect())
    }

    pub async fn get_current_hero_region(&self, hero_id: &str) -> Result<HeroRegion, QueryError> {
        let hero_region = db!()
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

        let actions = db!()
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
                let new_action = db!()
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
        let region = db!()
            .region()
            .create(
                region_name.to_str(),
                vec![adjacent_regions::set(adjacent_regions)],
            )
            .exec()
            .await?;

        Ok(region.into())
    }

    pub async fn get_npc_by_action_id(&self, action_id: &str) -> Result<Monster, QueryError> {
        let npc = db!()
            .npc()
            .find_first(vec![npc::WhereParam::ActionsSome(vec![
                action::id::equals(action_id.to_string()),
            ])])
            .exec()
            .await
            .unwrap()
            .unwrap();

        //we assume npc has a deck and cards when created

        let npc_obj: Monster = npc.into();

        Ok(npc_obj)
    }
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
