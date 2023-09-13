use std::{collections::HashMap, sync::Arc};

use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use prisma_client_rust::chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::{
    events::{dispatcher::EventDispatcher, game::GameEvent},
    services::{tasks::explore::ExploreAction, traits::hero_service::HeroService},
};
use crate::{infra::Infra, webserver::AppState};
use crate::{models::hero::Hero, services::impls::hero_service::ServiceHeroes};
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

#[post("/region/explore/{hero_id}")]
pub async fn explore_region(
    path: Path<String>,
    region_service: Data<Arc<RegionService>>,
    hero_service: Data<dyn HeroService>,
    app: Data<AppState>,
) -> impl Responder {
    let hero_id = path.into_inner();
    let hero = hero_service.get_hero(hero_id.clone()).await.unwrap();

    let current_region = region_service
        .get_hero_current_region(hero_id.clone())
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

pub fn do_explore(
    hero: &Hero,
    region_name: &RegionName,
    durations: &HashMap<RegionName, Duration>,
) -> Result<(), anyhow::Error> {
    let task = ExploreAction::new(hero.to_owned(), region_name.to_owned(), durations);
    match task {
        Some(task) => {
            //EVENT STUFF
            Infra::dispatch(GameEvent::HeroExplores(task.clone()));
            // END EVENT
            // let sent = app.executor.task_sender.send(GameEvent::Exploration(task));
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
