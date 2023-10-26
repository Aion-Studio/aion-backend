use actix_web::web::{Path, Query};
use actix_web::{get, post, HttpResponse, Responder};
use prisma_client_rust::chrono::{self, Local};
use prisma_client_rust::serde_json::json;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::events::game::{ActionCompleted, ActionNames, TaskAction};
use crate::infra::Infra;
use crate::models::hero::{Attributes, BaseStats, Hero, Range};
use crate::models::region::{HeroRegion, Leyline};
use crate::services::tasks::channel::ChannelingAction;
use crate::services::traits::async_task::Task;

#[derive(Serialize)]
struct HeroResponse {
    hero: Hero,
}

#[post("/heroes")]
async fn create_hero_endpoint() -> impl Responder {
    let mut rng = rand::thread_rng();

    let hero = Hero::new(
        BaseStats {
            id: None,
            level: 1,
            xp: 0,
            damage: Range {
                min: rng.gen_range(1..5),
                max: rng.gen_range(5..10),
            },
            hit_points: rng.gen_range(90..110),
            mana: rng.gen_range(40..60),
            armor: rng.gen_range(5..15),
        },
        Attributes {
            id: None,
            strength: rng.gen_range(1..20),
            resilience: rng.gen_range(1..20),
            agility: rng.gen_range(1..20),
            intelligence: rng.gen_range(1..20),
            exploration: rng.gen_range(1..20),
            crafting: rng.gen_range(1..20),
        },
        rng.gen_range(80..120),
        0,
    );

    let created_hero = Infra::repo().insert_hero(hero).await;

    match created_hero {
        Ok(hero) => {
            let hero_and_region = HeroResponse { hero };
            HttpResponse::Created().json(hero_and_region)
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeroStateResponse {
    hero: Hero,
    region_hero: HeroRegion,
    pub active_task: Option<TaskAction>,
    pub available_leylines: Vec<Leyline>,
    explore_available: (bool, chrono::DateTime<chrono::Local>),
    channeling_available: (Vec<Leyline>, chrono::DateTime<chrono::Local>),
}

#[get("/heroes/{id}")]
async fn hero_state(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await;

    info!("hero state requested....");
    match hero {
        Ok(hero) => match get_hero_status(hero).await {
            Ok(hero_state) => HttpResponse::Ok().json(hero_state),
            Err(e) => {
                let error_response = json!({
                    "error": "Error grabbing hero state",
                    "details": format!("{}", e)
                });
                HttpResponse::BadRequest().json(error_response)
            }
        },
        Err(e) => {
            let error_response = json!({
                "error": "Error grabbing hero state",
                "details": format!("{}", e)
            });
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

#[derive(Deserialize)]
struct PaginationQuery {
    take: Option<usize>,
    skip: Option<usize>,
}

#[get("/completed-actions")]
async fn completed_actions(pagination: Query<PaginationQuery>) -> impl Responder {
    let take = pagination.take.unwrap_or(10) as i64; // Default to 10 if not provided
    let skip = pagination.skip.unwrap_or(0) as i64;
    let actions = Infra::repo().completed_actions(take, skip).await;

    match actions {
        Ok(actions) => HttpResponse::Ok().json(actions),
        Err(e) => {
            let error_response = json!({
                "error": "Error grabbing completed actions",
                "details": format!("{}", e)
            });
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

#[get("/latest-action/{hero_id}")]
async fn latest_action_handler(hero_id: Path<String>) -> impl Responder {
    let hero_id = hero_id.into_inner();
    match Infra::repo().latest_action_completed(hero_id).await {
        Ok(action) => HttpResponse::Ok().json(action),
        Err(e) => {
            let error_response = json!({
                "error": "Error grabbing latest action",
                "details": format!("{}", e)
            });
            HttpResponse::BadRequest().json(error_response)
        }
    }
}

pub async fn get_hero_status(hero: Hero) -> Result<HeroStateResponse, anyhow::Error> {
    match Infra::repo().get_hero_regions(hero.get_id().as_ref()).await {
        Ok(hero_region) => {
            // find hero region with current_location true
            let current_region = hero_region
                .into_iter()
                .find(|hr| hr.current_location == true)
                .unwrap();
            let active_task = Infra::tasks().get_current_task(hero.get_id().as_ref());
            let available_leylines = Infra::repo()
                .leylines_by_discovery(&hero.get_id())
                .await
                .unwrap();

            let mut explore_available: Option<(bool, chrono::DateTime<chrono::Local>)> = None;

            let mut currently_channeling: Option<&ChannelingAction> = None;

            let leylines = Infra::repo()
                .leylines_by_discovery(&hero.get_id())
                .await
                .unwrap();

            if let Some(task) = &active_task {
                if let TaskAction::Explore(..) = task {
                    explore_available = Some((false, chrono::Utc::now().into()));
                }
                if let TaskAction::Channel(action) = task {
                    currently_channeling = Some(action);
                }
            }

            if explore_available.is_none() {
                explore_available = Some(
                    match Infra::repo()
                        .latest_action_of_type(hero.get_id(), ActionNames::Explore)
                        .await
                    {
                        Ok(latest_action) => {
                            if let Some(action) = latest_action {
                                let timeout_duration =
                                    hero.timeout_durations(&action.action_name).await;
                                let time_until_avialable = ActionCompleted::time_before_available(
                                    action.created_at.with_timezone(&Local),
                                    timeout_duration,
                                );
                                if let Some(time_until) = time_until_avialable {
                                    (false, chrono::Utc::now().with_timezone(&Local) + time_until)
                                } else {
                                    (true, chrono::Utc::now().with_timezone(&Local))
                                }
                            } else {
                                (true, chrono::Utc::now().with_timezone(&Local))
                            }
                        }
                        Err(e) => {
                            println!("Error getting latest action: {}", e);
                            (true, chrono::Utc::now().with_timezone(&Local))
                        }
                    },
                );
            }

            let explore_available = explore_available.unwrap();

            let channeling_available = {
                match currently_channeling {
                    Some(action) => {
                        let timeout_duration = hero.timeout_durations(&ActionNames::Channel).await;
                        let start_time = match action.start_time() {
                            Some(time) => Some(time),
                            None => None,
                        };
                        let time_until_avialable = ActionCompleted::time_before_available(
                            start_time.unwrap().with_timezone(&Local),
                            timeout_duration,
                        );
                        let now = chrono::Utc::now().with_timezone(&Local);
                        if let Some(time_until) = time_until_avialable {
                            (leylines, now + time_until)
                        } else {
                            (leylines, chrono::Utc::now().with_timezone(&Local))
                        }
                    }

                    None => {
                        match Infra::repo()
                            .latest_action_of_type(hero.get_id(), ActionNames::Channel)
                            .await
                        {
                            Ok(latest_action) => match latest_action {
                                Some(action) => {
                                    ActionCompleted::channeling_available(&action, &hero).await
                                }
                                None => (leylines, chrono::Utc::now().into()),
                            },
                            Err(e) => {
                                println!("Error getting latest action: {}", e);
                                (vec![], chrono::Utc::now().with_timezone(&Local))
                            }
                        }
                    }
                }
            };

            Ok(HeroStateResponse {
                hero,
                region_hero: current_region,
                active_task,
                available_leylines,
                explore_available,
                channeling_available,
            })
        }
        Err(err) => Err(err.into()),
    }
}
