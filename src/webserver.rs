use std::collections::HashMap;
use std::net::TcpListener;
use std::process::Command;
use std::sync::Arc;

use actix_cors::Cors;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::{self, Data, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::middleware::from_fn;
use alloy_primitives::Address;
use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};
use secrecy::{ExposeSecret, Secret};
use serde_json::json;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::authentication::reject_anonymous_users;
use crate::configuration::{get_durations, DurationType, Settings};
use crate::db_client::initialize_db;
use crate::endpoints::auth::login;
use crate::endpoints::cards::{
    add_card, add_to_deck, create_deck, get_cards, get_hero_cards, get_hero_decks, remove_card,
    remove_from_deck,
};
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

use crate::services::impls::combat_service::{CombatController, ControllerMessage};
use crate::session_state::TypedSession;
// use crate::storable::MemoryStore;

pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Clone)]
pub struct AppState {
    pub durations: HashMap<String, DurationType>,
    // pub combat_tx: mpsc::Sender<(CombatCommand, String, Sender<CombatTurnMessage>)>,
    pub combat_tx: mpsc::Sender<ControllerMessage>,
    pub signing_messages: HashMap<Address, String>,
}

impl AppState {
    pub fn signing_messages(&self) -> &HashMap<Address, String> {
        &self.signing_messages
    }
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

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let url = format!(
            "{}{}{}",
            configuration.database.url, configuration.database.name, configuration.database.params
        );

        let redis_uri = configuration.redis_uri.clone();
        let hmac_key = configuration.hmac_secret_key.clone();

        match initialize_db(url).await {
            Ok(_) => info!("Prisma client initialized"),
            Err(e) => error!("Prisma client failed to initialize: {:?}", e),
        }

        Infra::initialize();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, redis_uri, hmac_key).await?;
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

async fn run(
    listener: TcpListener,
    redis_uri: String,
    hmac_key: Secret<String>,
) -> Result<Server, anyhow::Error> {
    match Logger::init("127.0.0.1:9000") {
        Ok(_) => println!("Logger initialized at localhost:9000"),
        Err(e) => println!("Logger failed to initialize: {:?}", e),
    };

    // initialize the messenger
    let _ = MESSENGER;

    let redis_store = RedisSessionStore::new(redis_uri.clone()).await?;
    // Subscribe the task management service to the HeroExplored event

    // let store = Arc::new(Mutex::new(MemoryStore::new()));

    let (tx, rx) = mpsc::channel(1000);
    info!("___created new combat_tx____");

    let mut combat_controller = CombatController::new(tx.clone(), &redis_uri); // Use RwLock here

    // our combat runner
    tokio::spawn(async move {
        // This scope only needs a write lock briefly to start the `run` method
        combat_controller.run(rx).await;
        info!("combat run exiting...");
    });

    let app_state_s = AppState {
        durations: get_durations(),
        combat_tx: tx,
        signing_messages: HashMap::new(),
    };

    let app_state = Data::new(app_state_s.clone());

    let secret_key = Key::from(hmac_key.expose_secret().as_bytes());
    info!("connecting redis for session storage...{:?}", redis_uri);
    info!("connected to redis for session storage.");

    let server = HttpServer::new(move || {
        let cors = Cors::permissive()
            .allow_any_origin()
            // .supports_credentials() // This allows all origins. Adjust as needed.
            // .allowed_origin("http://localhost:9000")
            .max_age(3600);

        let app = App::new()
            .wrap(cors)
            // .service(login)
            // .wrap(SessionMiddleware::new(
            //     redis_store.clone(),
            //     secret_key.clone(),
            // ))
            // .service(logout)
            // .service(validate_session)
            .service(
                web::scope("/api")
                    // .wrap(from_fn(reject_anonymous_users))
                    .app_data(app_state.clone())
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
                    .service(add_to_deck)
                    .service(get_hero_decks)
                    .service(remove_from_deck)
                    .service(remove_card)
                    .service(get_hero_cards)
                    .service(get_hero_quests)
                    .service(do_quest_action)
                    .service(accept_quest)
                    .service(create_deck)
                    .service(npc)
                    .service(completed_actions),
            );
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[get("/validate-session")]
async fn validate_session(session: TypedSession) -> impl Responder {
    if session.is_active().unwrap_or(false) {
        HttpResponse::Ok().json("Session is active")
    } else {
        HttpResponse::Unauthorized().json("No active session")
    }
}

#[post("/logout")]
async fn logout(session: TypedSession) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().json("Logged out")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Hello from Rust"}))
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
    let heroes = Infra::hero_repo().get_all_heroes().await.unwrap();
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
