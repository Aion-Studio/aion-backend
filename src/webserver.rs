use crate::configuration::{get_durations, DurationType, Settings};
use crate::events::initialize::initialize_handlers;
use crate::handlers::heroes::{create_hero_endpoint, hero_state,  latest_action_handler};
use crate::handlers::regions::{channel_leyline, explore_region};
use crate::handlers::tasks::{active_actions, active_actions_ws, MyWebSocket};
use crate::infra::Infra;
use crate::logger::Logger;
use crate::prisma::PrismaClient;
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::{Data, Path};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub prisma: Arc<PrismaClient>,
    pub durations: HashMap<String, DurationType>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        //
        //concat database.url and database.name into one string joined by a "/"
        let url = format!(
            "{}{}",
            configuration.database.url, configuration.database.name
        );

        println!("prisma url {:?}", url);
        let prisma_result = PrismaClient::_builder().with_url(url).build().await;

        let prisma_client = match prisma_result {
            Ok(prisma_client) => prisma_client,
            Err(e) => {
                println!("Failed to connect to database: {:?}", e);
                return Err(anyhow::Error::new(e));
            }
        };
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address.clone())?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, prisma_client).await?;
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

async fn run(listener: TcpListener, prisma_client: PrismaClient) -> Result<Server, anyhow::Error> {
    let prisma = Arc::new(prisma_client);

    match Logger::init("http:://localhost:9000") {
        Ok(_) => println!("Logger initialized"),
        Err(e) => println!("Logger failed to initialize: {:?}", e),
    };

    // Subscribe the task management service to the HeroExplored event
    let app_state_s = AppState {
        prisma: prisma.clone(),
        durations: get_durations(),
    };
    let app_state = Data::new(app_state_s.clone());

    Infra::initialize(prisma.clone());

    initialize_handlers();
    let server = HttpServer::new(move || {
        let cors = Cors::permissive() // This allows all origins. Adjust as needed.
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            // .route("/health_check", web::get().to(health_check))
            // .route("/hero/actions", web::get().to(hero_actions))routes
            // .app_data(prisma.clone())
            .app_data(app_state.clone())
            .wrap(cors)
            .service(create_hero_endpoint)
            .service(explore_region)
            .service(channel_leyline)
            .service(get_heroes)
            .service(visible_leylines)
            .service(active_actions_ws)
            .service(active_actions)
            .service(latest_action_handler)
            .service(hero_state);
        // .service(add_leyline);
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[get("/all-heroes")]
async fn get_heroes() -> impl Responder {
    let heroes = Infra::repo().get_all_heroes().await.unwrap();
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
