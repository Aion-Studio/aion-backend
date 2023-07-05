use std::sync::Arc;

use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{
    models::region::{Leyline, RegionName},
    services::{impls::region_service::RegionServiceImpl, traits::region::RegionService},
};

#[derive(Debug, Deserialize)]
pub struct RegionPayload {
    region: RegionName,
    adjacent_regions: Vec<String>,
}

#[post("/region/explore/{hero_id}")]
pub async fn explore_region(
    path: Path<String>,
    region_service: Data<Arc<RegionServiceImpl>>,
) -> impl Responder {
    let hero_id = path.into_inner();

    let current_region = region_service
        .get_hero_current_region(hero_id.clone())
        .await
        .unwrap();

    let id = region_service.start_exploration(hero_id, current_region.region_name.clone());

    match id {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/region")]
pub async fn create_region(
    payload: Json<RegionPayload>,
    region_service: Data<RegionServiceImpl>,
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
    region_service: Data<RegionServiceImpl>,
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
