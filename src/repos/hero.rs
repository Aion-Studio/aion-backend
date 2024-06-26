use std::collections::HashMap;

use futures::future::join_all;
use prisma_client_rust::{chrono, Direction, QueryError};
use serde_json::Value;
use tracing::{error, info};

use crate::db;
use crate::models::resources::Resource;
use crate::prisma::{
    account, deck, hero_spell, resource_type, spell, stamina, Resource as PrismaResource,
};
use crate::utils::merge;
use crate::{
    events::game::ActionCompleted,
    models::{
        cards::{Card, Deck},
        hero::{convert_to_fixed_offset, Hero},
        region::RegionName,
    },
    prisma::{
        action_completed,
        deck::hero_id,
        hero::{self},
        hero_region::{self, current_location},
        hero_resource, region,
    },
    types::RepoFuture,
};

use super::cards::CardRepo;

pub struct HeroRepo {}

impl HeroRepo {
    pub async fn get_all_heroes(&self) -> Result<Vec<Value>, QueryError> {
        let heroes = db!()
            .hero()
            .find_many(vec![])
            .with(hero::hero_resources::fetch(vec![]).with(hero_resource::resource::fetch()))
            .with(hero::hero_region::fetch(vec![]))
            .with(hero::stamina::fetch())
            .with(hero::decks::fetch(vec![]))
            .with(hero::hero_cards::fetch(vec![]))
            .with(
                hero::hero_spells::fetch(vec![])
                    .with(hero_spell::spell::fetch().with(spell::effects::fetch(vec![]))),
            )
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
    pub async fn insert_hero(&self, new_hero: Hero) -> Result<Hero, QueryError> {
        // ...

        let rand_id = rand::random::<u64>().to_string();
        let account = db!().account().create(rand_id, vec![]).exec().await?;

        let stamina = db!().stamina().create(100, 100, vec![]).exec().await?;

        let result = db!()
            .hero()
            .create(
                new_hero.name,
                new_hero.class,
                new_hero.hp,
                new_hero.strength,
                new_hero.dexterity,
                new_hero.intelligence,
                new_hero.explore,
                new_hero.crafting,
                new_hero.armor,
                1,
                vec![
                    hero::stamina::connect(stamina::id::equals(stamina.id)),
                    hero::account::connect(account::id::equals(account.id)),
                ],
            )
            .with(hero::stamina::fetch())
            .exec()
            .await?;

        let hero: Hero = result.into();

        db!()
            .deck()
            .create(vec![hero_id::set(Some(hero.get_id()))])
            .exec()
            .await?;
        let region_name = RegionName::Dusane;

        db!()
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
                                    db!()
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
        let hero_data = db!()
            .hero()
            .find_unique(hero::id::equals(hero_id.clone()))
            .with(
                hero::hero_spells::fetch(vec![hero_spell::hero_id::equals(hero_id.clone())])
                    .with(hero_spell::spell::fetch().with(spell::effects::fetch(vec![]))),
            )
            .with(hero::decks::fetch(vec![deck::hero_id::equals(Some(
                hero_id.clone(),
            ))]))
            .with(hero::hero_region::fetch(vec![
                hero_region::hero_id::equals(hero_id.clone()),
            ]))
            .with(
                // fetch resources where the hero_resource id is our hero, then
                // include the resource for each hero_resource
                hero::hero_resources::fetch(vec![hero_resource::hero_id::equals(hero_id.clone())])
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
    }

    pub async fn update_hero(&self, hero: Hero) -> Result<Hero, QueryError> {
        self.update_hero_resources(&hero.resources, String::from(&hero.get_id()))
            .await?;

        db!()
            .stamina()
            .update(
                stamina::hero_id::equals(hero.get_id()),
                vec![
                    stamina::amount::set(hero.stamina.amount),
                    stamina::capacity::set(hero.stamina.capacity),
                    stamina::last_regen_time::set(convert_to_fixed_offset(
                        hero.stamina.last_regen_time,
                    )),
                ],
            )
            .exec()
            .await?;

        let updated_hero = db!()
            .hero()
            .update(
                hero::id::equals(hero.get_id()),
                vec![
                    hero::hp::set(hero.hp),
                    hero::strength::set(hero.strength),
                    hero::dexterity::set(hero.dexterity),
                    hero::intelligence::set(hero.intelligence),
                    hero::explore::set(hero.explore),
                    hero::crafting::set(hero.crafting),
                    hero::armor::set(hero.armor),
                    hero::level::set(hero.level),
                ],
            )
            .with(
                hero::hero_resources::fetch(vec![hero_resource::hero_id::equals(hero.get_id())])
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
                let resource_enum = PrismaResource::from(resource.clone());
                let prisma = db!().clone();
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
}
