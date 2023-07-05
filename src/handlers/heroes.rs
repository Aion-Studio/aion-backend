use std::sync::Arc;

use actix_web::web::{Data, Path};
use actix_web::{get, post, HttpResponse, Responder};
use serde::Serialize;

use crate::models::hero::Hero;
use crate::models::region::HeroRegion;
use crate::models::task::TaskKind;
use crate::services::impls::game_engine_service::GameEngineService;
use crate::services::impls::hero_service::ServiceHeroes;
use crate::services::impls::region_service::RegionServiceImpl;
use crate::services::impls::task_scheduler_service::TaskSchedulerService;
use crate::services::traits::game_engine::GameEngine;
use crate::services::traits::hero_service::HeroService;
use crate::services::traits::region::RegionService;
use crate::services::traits::scheduler::TaskScheduler;

#[derive(Serialize)]
struct HeroResponse {
    hero: Hero,
    region_hero: HeroRegion,
}

#[post("/heroes")]
async fn create_hero_endpoint(
    hero_service: Data<ServiceHeroes>,
    region_service: Data<RegionServiceImpl>,
    game_engine: Data<GameEngineService>,
) -> impl Responder {
    let hero_data = game_engine.generate_hero().await.unwrap();
    let created_hero = hero_service.create_hero(hero_data.clone()).await.unwrap();
    let region_hero = region_service.create_region_hero(&created_hero).await;

    match region_hero {
        Ok(region_hero) => {
            let hero_and_region = HeroResponse {
                hero: created_hero,
                region_hero,
            };
            HttpResponse::Created().json(hero_and_region)
        }
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[derive(Serialize)]
struct HeroStateResponse {
    hero: Hero,
    region_hero: HeroRegion,
    active_task: Option<TaskKind>,
}

#[get("/heroes/{id}")]
async fn hero_state(
    hero_service: Data<ServiceHeroes>,
    region_service: Data<Arc<RegionServiceImpl>>,
    task_scheduler: Data<Arc<TaskSchedulerService>>, // to this line
    path: Path<String>,
) -> impl Responder {
    let hero_id = path.into_inner();
    let current_region = region_service
        .get_hero_current_region(hero_id.clone())
        .await
        .unwrap();
    let current_task = task_scheduler.get_current_task(&hero_id);
    let hero = hero_service.get_hero(hero_id.clone()).await.unwrap();

    HttpResponse::Ok().json(HeroStateResponse {
        hero,
        region_hero: current_region,
        active_task: current_task,
    })
}
