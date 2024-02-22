use actix_web::web::{Data, Path, Query};
use actix_web::{get, post, HttpResponse, Responder};
use prisma_client_rust::chrono::{self, Local};
use prisma_client_rust::serde_json::json;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tracing::{error, info};

use crate::events::game::ActionCompleted;
use crate::infra::Infra;
use crate::models::hero::{Attributes, BaseStats, Hero, Range};
use crate::models::quest::{HeroQuest, Quest};
use crate::models::region::{HeroRegion, Leyline};
use crate::services::impls::combat_service::ControllerMessage;
use crate::services::tasks::action_names::{ActionNames, TaskAction};
use crate::services::tasks::channel::ChannelingAction;
use crate::services::tasks::explore::ExploreAction;
use crate::services::traits::async_task::Task;
use crate::webserver::AppState;

#[derive(Serialize)]
struct HeroResponse {
    hero: Hero,
}

#[post("/heroes")]
async fn create_hero_endpoint() -> impl Responder {
    let mut rng = rand::thread_rng();
    let mut hp = rng.gen_range(475..725);
    let damage_min = rng.gen_range(21..28); // Adjusted for "low end" specification
    let damage_max = rng.gen_range(26..49); // Adjusted for "high end" specification
    let mut armor = rng.gen_range(2..6);
    let mut strength = rng.gen_range(15..26);
    let mut intelligence = rng.gen_range(15..20); // Adjusted upper bound to 20 for inclusivity
    let mut agility = rng.gen_range(10..25);
    let mut exploration = rng.gen_range(1..20);
    let mut crafting = rng.gen_range(1..20);
    // Determine category based on HP
    let category = if hp >= 650 {
        // High HP
        agility = rng.gen_range(10..15); // Lower agility for high HP
        intelligence = rng.gen_range(15..17); // Low-medium intelligence
        strength = rng.gen_range(20..26); // High strength
        "High HP"
    } else if hp < 500 {
        // Low HP
        intelligence = rng.gen_range(17..20); // High intelligence
        exploration = rng.gen_range(15..20); // High exploration
        crafting = rng.gen_range(15..20); // High crafting
        agility = rng.gen_range(15..25); // High agility
        armor = rng.gen_range(3..6); // Medium armor
        "Low HP"
    } else if agility > 21 {
        // High agility
        hp = rng.gen_range(475..550); // Medium-low HP
        armor = rng.gen_range(4..6); // High armor
        intelligence = rng.gen_range(15..18); // Medium intelligence
        "High Agility"
    } else {
        // Medium agility and low HP
        // Adjusting attributes to top 90% of given ranges
        strength = rng.gen_range(22..26);
        intelligence = rng.gen_range(17..20);
        exploration = rng.gen_range(18..20);
        crafting = rng.gen_range(18..20);
        "Medium Agility & Low HP"
    };
    let hero = Hero::new(
        BaseStats {
            id: None,
            level: 1,
            xp: 0,
            damage: Range {
                min: damage_min,
                max: damage_max,
            },
            hit_points: hp,
            armor,
        },
        Attributes {
            id: None,
            strength,
            resilience: rng.gen_range(1..20), // Unspecified, so left as is
            agility,
            intelligence,
            exploration,
            crafting,
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
    // explore_available returns None if there isnt enough stamina, and (bool,time) if there is a
    // timeout since last explore
    explore_available: Option<(bool, chrono::DateTime<Local>)>,
    explore_cost: Option<i32>,
    channeling_available: (Vec<Leyline>, chrono::DateTime<Local>),
    quest_accepted: (bool, Quest),
    is_in_combat: bool, // if hero is currently in a combat encounter
}

#[get("/heroes/{id}")]
async fn hero_state(path: Path<String>, app_state: Data<AppState>) -> impl Responder {
    let hero_id = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await;

    info!("hero state requested....");
    let app_state = app_state.get_ref().clone();
    let combat_tx = app_state.combat_tx.clone();

    match hero {
        Ok(hero) => match get_hero_status(hero, combat_tx).await {
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
    info!("completed actions requested....");
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

pub async fn get_hero_status(
    hero: Hero,
    combat_tx: Sender<ControllerMessage>,
) -> Result<HeroStateResponse, anyhow::Error> {
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
            let mut explore_cost = None;

            let leylines = Infra::repo()
                .leylines_by_discovery(&hero.get_id())
                .await
                .unwrap();

            if let Some(task) = &active_task {
                if let TaskAction::Channel(action) = task {
                    currently_channeling = Some(&action);
                }
            }

            // retreieve latest available explore action
            let hero_action_explore = Infra::repo()
                .get_or_create_hero_explore_action(hero.get_id().as_ref())
                .await;

            if hero_action_explore.is_ok() {
                // checks if hero has enough stamina and set the cost inside constructor
                match ExploreAction::new(
                    hero.clone(),
                    current_region.clone(),
                    hero_action_explore.unwrap().cost.unwrap(),
                ) {
                    Some(explore_action) => {
                        explore_cost = Some(explore_action.stamina_cost);
                        explore_available = Some(
                            match Infra::repo()
                                .latest_action_of_type(hero.get_id(), ActionNames::Explore)
                                .await
                            {
                                Ok(latest_action) => {
                                    if let Some(action) = latest_action {
                                        let timeout_duration =
                                            hero.timeout_durations(&action.action_name).await;
                                        let time_until_avialable =
                                            ActionCompleted::time_before_available(
                                                action.created_at.with_timezone(&Local),
                                                timeout_duration,
                                            );
                                        if let Some(time_until) = time_until_avialable {
                                            (
                                                false,
                                                chrono::Utc::now().with_timezone(&Local)
                                                    + time_until,
                                            )
                                        } else {
                                            (true, chrono::Utc::now().with_timezone(&Local))
                                        }
                                    } else {
                                        (true, chrono::Utc::now().with_timezone(&Local))
                                    }
                                }
                                Err(e) => {
                                    error!("Error getting latest action: {}", e);
                                    (true, chrono::Utc::now().with_timezone(&Local))
                                }
                            },
                        );
                    }
                    None => {
                        explore_available = None;
                    }
                };
            }

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
                            Err(e) => (vec![], chrono::Utc::now().with_timezone(&Local)),
                        }
                    }
                }
            };

            let hero_quest_objs = match Infra::repo().get_quest_by_hero_id(hero.get_id()).await {
                Ok(objs) => Some(objs),
                Err(e) => {
                    error!("Error getting quest: {}", e);
                    None
                }
            };

            // check if combat in play
            let (tx, rx) = oneshot::channel();

            let msg = ControllerMessage::RequestState {
                combatant_id: hero.get_id().clone(),
                tx,
            };
            combat_tx.send(msg).await.unwrap();
            let is_in_combat = match rx.await {
                Ok(res) => res.is_some(),
                Err(e) => {
                    error!("Error getting combat state: {}", e);
                    false
                }
            };

            Ok(HeroStateResponse {
                hero,
                region_hero: current_region,
                active_task,
                available_leylines,
                explore_available,
                explore_cost,
                is_in_combat,
                channeling_available,
                quest_accepted: match hero_quest_objs {
                    Some((quest, hero_quest)) => {
                        if hero_quest.accepted {
                            (true, quest)
                        } else {
                            (false, Quest::default())
                        }
                    }
                    None => (false, Quest::default()),
                },
            })
        }
        Err(err) => Err(err.into()),
    }
}
