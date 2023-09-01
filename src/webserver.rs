use crate::configuration::{get_region_durations, Durations, Settings};
use crate::handlers::heroes::{create_hero_endpoint, hero_state};
use crate::handlers::regions::{create_region, explore_region};
use crate::prisma::PrismaClient;
use crate::services::impls::action_executor::ActionExecutor;
use crate::services::impls::hero_service::ServiceHeroes;
use crate::services::impls::region_service::RegionService;
use crate::services::impls::tasks::TaskManager;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;
use tracing::info;

pub struct Application {
    port: u16,
    server: Server,
}

#[derive(Clone)]
pub struct AppState {
    pub prisma: Arc<PrismaClient>,
    pub executor: Arc<ActionExecutor>,
    pub durations: Durations,
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

    let hero_service = web::Data::new(ServiceHeroes::new(prisma.clone()));
    let executor = ActionExecutor::new(prisma.clone());
    let scheduler = Arc::new(TaskManager::new());
    let task_schedule_service = web::Data::new(scheduler.clone());
    let region_service = web::Data::new(RegionService::new(
        scheduler,
        prisma.clone(),
        executor.clone().result_channels().unwrap(),
    ));
    let app_state = web::Data::new(AppState {
        prisma: prisma.clone(),
        executor: executor.clone(),
        durations: get_region_durations(),
    });

    let server = HttpServer::new(move || {
        let app = App::new()
            // .route("/health_check", web::get().to(health_check))
            // .route("/hero/actions", web::get().to(hero_actions))routes
            // .app_data(prisma.clone())
            .app_data(hero_service.clone())
            .app_data(app_state.clone())
            .app_data(task_schedule_service.clone())
            .app_data(region_service.clone())
            .service(create_hero_endpoint)
            .service(explore_region)
            .service(hero_state)
            .service(create_region);
        // .service(add_leyline);
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}
