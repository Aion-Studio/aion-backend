use std::{collections::HashMap, sync::Arc};

use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use prisma_client_rust::chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::configuration::{ChannelDurations, DurationType};
use crate::services::tasks::channel::ChannelingAction;
use crate::{
    events::{dispatcher::EventDispatcher, game::GameEvent},
    models::hero::Hero,
    services::{tasks::explore::ExploreAction, traits::hero_service::HeroService},
};
use crate::{infra::Infra, webserver::AppState};
use crate::{
    models::region::{Leyline, RegionName},
    services::impls::region_service::RegionService,
};

#[derive(Debug, Deserialize)]
pub struct RegionPayload {
    region: RegionName,
    adjacent_regions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ExploreResponse {
    message: String,
    status: String,
}

#[get("/region/explore/{hero_id}")]
pub async fn explore_region(path: Path<String>, app: Data<AppState>) -> impl Responder {
    let hero_id = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();

    let current_region = Infra::repo()
        .get_current_hero_region(&hero_id)
        .await
        .unwrap();

    match do_explore(&hero, &current_region.region_name, &app.durations) {
        Ok(_) => HttpResponse::Ok().json(ExploreResponse {
            message: "Exploration started".to_string(),
            status: "Ok".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ExploreResponse {
            message: e.to_string(),
            status: "Error".to_string(),
        }),
    }
}

#[get("/region/channel/{leyline_name}/{hero_id}")]
pub async fn channel_leyline(path: Path<(String, String)>, app: Data<AppState>) -> impl Responder {
    let (leyline_name, hero_id) = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();

    assert!(hero.can_channel(&leyline_name).await);

    let durations = match app.durations.get("Channel") {
        Some(DurationType::Channel(durations)) => Ok(durations.clone()),
        _ => Err(anyhow::Error::msg("No explore durations found")),
    }
    .unwrap();

    match do_channel(&hero, &leyline_name, &durations) {
        Ok(_) => HttpResponse::Ok().json(ExploreResponse {
            message: "Channeling started".to_string(),
            status: "Ok".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ExploreResponse {
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

pub fn do_explore(
    hero: &Hero,
    region_name: &RegionName,
    durations: &HashMap<String, DurationType>,
) -> Result<(), anyhow::Error> {
    let explore_durations = match durations.get("Explore") {
        Some(DurationType::Explore(durations)) => Ok(durations.clone()),
        _ => Err(anyhow::Error::msg("No explore durations found")),
    };

    //if explore_duratiosn is error return error right away
    let explore_durations = explore_durations?;

    let task = ExploreAction::new(hero.to_owned(), region_name.to_owned(), &explore_durations);
    match task {
        Some(task) => {
            Infra::dispatch(GameEvent::HeroExplores(task.clone()));
            Ok(())
        }
        None => Err(anyhow::Error::msg("Not enough stamina")),
    }
}

#[post("/region")]
pub async fn create_region(
    payload: Json<RegionPayload>,
    region_service: Data<RegionService>,
) -> impl Responder {
    let created_region = region_service
        .insert_new_region(payload.region.clone(), payload.adjacent_regions.clone())
        .await;

    match created_region {
        Ok(region) => HttpResponse::Created().json(region),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }
}
