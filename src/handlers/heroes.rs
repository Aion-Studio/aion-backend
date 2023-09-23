use std::sync::Arc;

use actix_web::web::{Data, Path};
use actix_web::{get, post, HttpResponse, Responder};
use prisma_client_rust::serde_json::json;
use prisma_client_rust::QueryError;
use rand::Rng;
use serde::Serialize;

use crate::events::game::GameEvent;
use crate::infra::Infra;
use crate::models::hero::{Attributes, BaseStats, Hero, Range};
use crate::models::region::{HeroRegion, Leyline, Region};
use crate::services::impls::region_service::RegionService;
use crate::services::impls::tasks::TaskManager;
use crate::services::traits::hero_service::HeroService;

#[derive(Serialize)]
struct HeroResponse {
    hero: Hero,
    region_hero: HeroRegion,
}

#[post("/heroes")]
async fn create_hero_endpoint(region_service: Data<RegionService>) -> impl Responder {
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

    let created_hero = Infra::repo().insert_hero(hero).await.unwrap();
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

#[derive(Serialize, Debug)]
pub struct HeroStateResponse {
    hero: Hero,
    region_hero: HeroRegion,
    pub active_task: Option<GameEvent>,
    pub available_leylines: Vec<Leyline>,
}

#[get("/heroes/{id}")]
async fn hero_state(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
    match get_hero_status(hero).await {
        Ok(hero_state) => HttpResponse::Ok().json(hero_state),
        Err(e) => {
            let error_response = json!({
                "error": "Error grabbing hero state",
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

            Ok(HeroStateResponse {
                hero,
                region_hero: current_region,
                active_task,
                available_leylines,
            })
        }
        Err(err) => Err(err.into()),
    }
}
