use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use tokio::sync::oneshot;
use tracing::info;

use crate::models::hero::Hero;
use crate::{
    configuration::{ChannelDurations, DurationType},
    messenger::MESSENGER,
    services::tasks::action_names::{Command, TaskAction},
};
use crate::{endpoints::response::ApiResponse, models::region::RegionName};
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

    info!("inside handler before do_exploore");
    match do_explore(hero, current_region.region_name).await {
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

    match do_channel(hero, leyline_name, durations).await {
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

pub async fn do_channel(
    hero: Hero,
    leyline_name: String,
    durations: ChannelDurations,
) -> Result<(), anyhow::Error> {
    let response = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Channel {
            hero_id: hero.get_id(),
            leyline_name: leyline_name.clone(),
            durations,
            resp: resp_tx,
        };
        MESSENGER.send(cmd);
        let res = resp_rx.await;
        res
    });
    match response.await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => {
            Err(anyhow::Error::msg(format!("Error starting channel: {}", e.to_string())).into())
        }
        Err(e) => Err(anyhow::Error::msg(e.to_string())),
    }
}

pub async fn do_explore(hero: Hero, region_name: RegionName) -> Result<(), anyhow::Error> {
    let response = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let hero_id = hero.get_id();
        let cmd = Command::Explore {
            hero_id: hero_id.clone(),
            region_name: region_name.clone(),
            resp: resp_tx,
        };

        MESSENGER.send(cmd);

        let res = resp_rx.await;
        res
    });

    match response.await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => {
            Err(anyhow::Error::msg(format!("Error starting explore: {}", e.to_string())).into())
        }
        Err(e) => Err(anyhow::Error::msg(e.to_string())),
    }
}
