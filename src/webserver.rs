use std::collections::HashMap;
use std::net::TcpListener;
use std::process::Command;
use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::{self, Data, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;
use tracing::info;

use crate::configuration::{get_durations, DurationType, Settings};
use crate::endpoints::combat_socket::combat_ws;
use crate::endpoints::heroes::{
    completed_actions, create_hero_endpoint, hero_state, latest_action_handler,
};
use crate::endpoints::quest::{accept_quest, add_quest, do_quest_action, get_hero_quests};
use crate::endpoints::regions::{channel_leyline, explore_region};
use crate::endpoints::tasks::{active_actions, active_actions_ws};
use crate::infra::Infra;
use crate::logger::Logger;
use crate::messenger::MESSENGER;
use crate::prisma::PrismaClient;
use crate::repos::cards::CardRepo;
use crate::services::impls::combat_service::{CombatController, ControllerMessage};
use crate::storable::MemoryStore;

pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Clone)]
pub struct AppState {
    pub durations: HashMap<String, DurationType>,
    // pub combat_tx: mpsc::Sender<(CombatCommand, String, Sender<CombatTurnMessage>)>,
    pub combat_tx: mpsc::Sender<ControllerMessage>,
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

    let store = Arc::new(Mutex::new(MemoryStore::new()));

    let (tx, rx) = mpsc::channel(1000);
    info!("___created new combat_tx____");

    let mut combat_controller = CombatController::new(tx.clone()); // Use RwLock here

    // our combat runner
    tokio::spawn(async move {
        // This scope only needs a write lock briefly to start the `run` method
        combat_controller.run(rx).await;
        info!("combat run exiting...");
    });

    let app_state_s = AppState {
        durations: get_durations(),
        combat_tx: tx,
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
            .app_data(store.clone())
            .wrap(cors)
            .service(health_check)
            .service(create_hero_endpoint)
            .service(explore_region)
            .service(channel_leyline)
            .service(get_heroes)
            .service(visible_leylines)
            .service(active_actions_ws)
            .service(combat_ws)
            .service(active_actions)
            .service(latest_action_handler)
            .service(hero_state)
            .service(add_quest)
            .service(get_cards)
            .service(add_card)
            .service(get_hero_cards)
            .service(get_hero_quests)
            .service(do_quest_action)
            .service(accept_quest)
            .service(npc)
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

#[get("/npc/{action_id}")]
async fn npc(path: Path<String>) -> impl Responder {
    let action_id = path.into_inner();
    let npc = Infra::repo()
        .get_npc_by_action_id(&action_id)
        .await
        .unwrap();
    HttpResponse::Ok().json(npc)
}

#[get("/all-heroes")]
async fn get_heroes() -> impl Responder {
    let heroes = Infra::repo().get_all_heroes().await.unwrap();
    HttpResponse::Ok().json(heroes)
}

#[get("/all-cards")]
async fn get_cards() -> impl Responder {
    let cards = CardRepo::get_all_cards().await;
    HttpResponse::Ok().json(cards)
}

// grab the request body hero_id and card_id
#[derive(serde::Deserialize)]
pub struct AddCardRequest {
    hero_id: String,
    card_id: String,
}

#[post("/add-card")]
async fn add_card(action: web::Json<AddCardRequest>) -> impl Responder {
    let hero_id = action.hero_id.clone();
    let card_id = action.card_id.clone();
    let card = CardRepo::add_card(hero_id, card_id).await;
    HttpResponse::Ok().json(card)
}

#[get("/hero-cards/{hero_id}")]
async fn get_hero_cards(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    let cards = CardRepo::get_all_hero_cards(hero_id).await;
    HttpResponse::Ok().json(cards)
}

#[get("/visible-leylines/{id}")]
async fn visible_leylines(path: Path<String>) -> impl Responder {
    let leylines = Infra::repo()
        .leylines_by_discovery(&path.into_inner())
        .await
        .unwrap();
    HttpResponse::Ok().json(leylines)
}
