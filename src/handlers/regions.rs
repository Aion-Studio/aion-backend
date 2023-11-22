use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Serialize;

use crate::{models::region::RegionName, handlers::response::ApiResponse};
use crate::services::tasks::channel::ChannelingAction;
use crate::{
    configuration::{ChannelDurations, DurationType, ExploreDurations},
    events::game::TaskAction,
};
use crate::{events::game::GameEvent, models::hero::Hero, services::tasks::explore::ExploreAction};
use crate::{infra::Infra, webserver::AppState};

#[get("/region/explore/{hero_id}")]
pub async fn explore_region(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();

    let active_tasks = Infra::tasks().get_current_task(hero_id.as_ref());

    if let Some(task) = active_tasks {
        if let TaskAction::Explore(..) = task {
            return HttpResponse::Forbidden().json(ApiResponse {
                message: "Already exploring".to_string(),
                status: "Error".to_string(),
            });
        }
    }

    let current_region = Infra::repo()
        .get_current_hero_region(&hero_id)
        .await
        .unwrap();

    match do_explore(&hero, &current_region.region_name) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            message: "Exploration started".to_string(),
            status: "Ok".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            message: e.to_string(),
            status: "Error".to_string(),
        }),
    }
}

#[get("/region/channel/{leyline_name}/{hero_id}")]
pub async fn channel_leyline(path: Path<(String, String)>, app: Data<AppState>) -> impl Responder {
    let (leyline_name, hero_id) = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();

    if !hero.can_channel(&leyline_name).await {
        return HttpResponse::Forbidden().json(ApiResponse {
            message: "Can't channel on this leyline".to_string(),
            status: "Error".to_string(),
        });
    };

    let durations = match app.durations.get("Channel") {
        Some(DurationType::Channel(durations)) => Ok(durations.clone()),
        _ => Err(anyhow::Error::msg("No explore durations found")),
    }
    .unwrap();

    match do_channel(&hero, &leyline_name, &durations) {
        Ok(_) => HttpResponse::Ok().json(ApiResponse {
            message: "Channeling started".to_string(),
            status: "Ok".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse {
            message: e.to_string(),
            status: "Error".to_string(),
        }),
    }
}

pub fn do_channel(
    hero: &Hero,
    leyline_name: &str,
    durations: &ChannelDurations,
) -> Result<(), anyhow::Error> {
    let task = ChannelingAction::new(hero.to_owned(), leyline_name, &durations);
    match task {
        Some(task) => {
            Infra::dispatch(GameEvent::Channeling(task.clone()));
            Ok(())
        }
        None => Err(anyhow::Error::msg("Not enough stamina")),
    }
}

pub fn do_explore(hero: &Hero, region_name: &RegionName) -> Result<(), anyhow::Error> {
    let task = ExploreAction::new(hero.to_owned(), region_name.to_owned());
    match task {
        Some(task) => {
            Infra::dispatch(GameEvent::HeroExplores(task.clone()));
            Ok(())
        }
        None => Err(anyhow::Error::msg("Not enough stamina")),
    }
}
