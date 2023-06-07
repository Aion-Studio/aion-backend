use crate::configuration::Settings;
use crate::prisma::PrismaClient;
use crate::services::impls::hero_service::ServiceHeroes;
// use crate::routes::{health_check, hero_actions};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
// use actix_web_lab::middleware::from_fn;
use std::net::TcpListener;

use tracing::info;

pub struct Application {
    port: u16,
    server: Server,
}



impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        //
        //concat database.url and database.name into one string joined by a "/"
        let url = format!(
            "{}{}",
            configuration.database.url, configuration.database.name
        );

        info!("prisma url {:?}", url);
        let prisma_result = PrismaClient::_builder().with_url(url).build().await;

        let prisma_client = match prisma_result {
            Ok(prisma_client) => prisma_client,
            Err(e) => {
                tracing::error!("Failed to connect to database: {:?}", e);
                return Err(anyhow::Error::new(e));
            }
        };
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, prisma_client).await?;
        info!(
            ".......................Server running on port {}........................",
            port
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
    let prisma = web::Data::new(prisma_client);

    let hero_service = Box::new(ServiceHeroes::new(prisma.clone()));
    let server = HttpServer::new(move || {
        App::new()
            // .route("/health_check", web::get().to(health_check))
            // .route("/hero/actions", web::get().to(hero_actions))routes
            .app_data(prisma.clone())
            .app_data(hero_service.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
