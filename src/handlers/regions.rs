use std::sync::Arc;

use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::models::task::TaskKind;
use crate::services::tasks::explore::ExploreAction;
use crate::webserver::AppState;
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
    app: Data<AppState>,
) -> impl Responder {
    let hero_id = path.into_inner();

    let current_region = region_service
        .get_hero_current_region(hero_id.clone())
        .await
        .unwrap();

    let task = ExploreAction::new(hero_id, current_region.region_name, &app.durations);
    let sent = app.executor.task_sender.send(TaskKind::Exploration(task));

    match sent {
        Ok(()) => HttpResponse::Ok().json(ExploreResponse {
            message: "Exploration started".to_string(),
            status: "OK".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
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

#[post("/region/{region_name}/leyline")]
pub async fn add_leyline(
    path: Path<String>,
    leyline: Json<Leyline>,
    region_service: Data<RegionService>,
) -> impl Responder {
    let region_name = path.into_inner();
    let created_leyline = region_service
        .insert_leyline(
            region_name.parse().unwrap(),
            leyline.location.clone(),
            leyline.xp_reward.clone(),
        )
        .await;
    match created_leyline {
        Ok(leyline) => HttpResponse::Created().json(leyline),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }
}
