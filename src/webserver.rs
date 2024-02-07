use crate::configuration::{get_durations, DurationType, Settings};
use crate::handlers::heroes::{
    completed_actions, create_hero_endpoint, hero_state, latest_action_handler,
};
use crate::handlers::quest::{add_quest, do_quest_action, get_hero_quests, accept_quest};
use crate::handlers::regions::{channel_leyline, explore_region};
use crate::handlers::tasks::{active_actions, active_actions_ws};
use crate::infra::Infra;
use crate::logger::Logger;
use crate::messenger::MESSENGER;
use crate::prisma::PrismaClient;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::{Data, Path};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::net::TcpListener;
use std::process::Command;
use std::sync::Arc; use tracing::info; pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub durations: HashMap<String, DurationType>,
}

#[allow(dead_code)]
fn run_prisma_migrations(config: &Settings) -> Result<(), std::io::Error> {
    let db_url = format!(
        "{}{}{}",
        config.database.url, config.database.name, config.database.params
    );
    println!("db url {:?}", db_url);
    let status = Command::new("cargo")
        .arg("prisma")
        .arg("migrate")
        .arg("dev")
        .env("DATABASE_URL", db_url)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Prisma migration failed",
        ))
    }
}

pub static PRISMA_CLIENT: OnceCell<Arc<PrismaClient>> = OnceCell::new();

pub fn get_prisma_client() -> Arc<PrismaClient> {
    PRISMA_CLIENT
        .get()
        .expect("Prisma client has not been initialized")
        .clone()
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            configuration.database.url, configuration.database.name, configuration.database.params
        );

        let prisma_client = PrismaClient::_builder()
            .with_url(url)
            .build()
            .await
            .expect("Failed to connect to database");

        PRISMA_CLIENT
            .set(Arc::new(prisma_client))
            .expect("Failed to set the global Prisma client");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener).await?;
        info!(
            "... ....................Server started at {} and port {}........................",
            address, port
        );

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    match Logger::init("127.0.0.1:9000") {
        Ok(_) => println!("Logger initialized at localhost:9000"),
        Err(e) => println!("Logger failed to initialize: {:?}", e),
    };

    // initialize the messenger
    let _ = MESSENGER;

    // Subscribe the task management service to the HeroExplored event
    let app_state_s = AppState {
        durations: get_durations(),
    };
    let app_state = Data::new(app_state_s.clone());

    Infra::initialize();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive() // This allows all origins. Adjust as needed.
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .service(health_check)
            .service(create_hero_endpoint)
            .service(explore_region)
            .service(channel_leyline)
            .service(get_heroes)
            .service(visible_leylines)
            .service(active_actions_ws)
            .service(active_actions)
            .service(latest_action_handler)
            .service(hero_state)
            .service(add_quest)
            .service(get_hero_quests)
            .service(do_quest_action)
            .service(accept_quest)
            .service(completed_actions);
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[get("/up")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/all-heroes")]
async fn get_heroes() -> impl Responder {
    info!("Fetching all heroes....");
    let heroes = Infra::repo().get_all_heroes().await.unwrap();
    info!("Successfully fetched all heroes....");
    HttpResponse::Ok().json(heroes)
}

#[get("/visible-leylines/{id}")]
async fn visible_leylines(path: Path<String>) -> impl Responder {
    let leylines = Infra::repo()
        .leylines_by_discovery(&path.into_inner())
        .await
        .unwrap();
    HttpResponse::Ok().json(leylines)
}
