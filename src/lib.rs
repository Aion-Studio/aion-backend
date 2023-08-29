// pub mod authentication;
pub mod configuration;
// pub mod idempotency;
// pub mod routes;
// pub mod session_state;
pub mod types;
pub mod utils;
pub mod webserver;

pub mod telemetry;

#[allow(warnings, unused)]
pub mod prisma;

#[allow(dead_code)]
#[allow(unused_variables)]
mod models {
    pub mod date_times;
    pub mod game_engine;
    pub mod hero;
    pub mod region;
    pub mod resources;
    pub mod task;
}

mod repos {
    pub mod game_engine;
    pub mod hero_repo;
    pub mod region_repo;
    // pub mod action_repo;
    // pub mod resources_repo;
}

mod services {
    pub mod impls {
        pub mod action_executor;
        pub mod hero_service;
        pub mod region_service;

        #[cfg(test)]
        mod hero_service_test;

        #[cfg(test)]
        mod region_service_test;

        pub mod items_service;
        pub mod tasks;
    }

    pub mod traits {
        pub mod async_task;
        pub mod hero_service;
        pub mod scheduler;
    }
    pub mod tasks {
        pub mod explore;
    }
}

pub mod handlers {
    pub mod heroes;
    pub mod regions;
}

#[cfg(test)]
pub mod test_helpers;

const LOG_ENV_VAR: &str = "INDEXER_LOG";
#[cfg(not(all(tokio_unstable, feature = "debug")))]
pub fn tracing_subscribe() -> bool {
    use std::env::{set_var, var};

    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    if var("INDEXER_LOG").is_err() {
        set_var("INDEXER_LOG", "debug");
    }

    println!("Tracing subscriber initialized");
    let env_filter = fmt::layer().with_filter(EnvFilter::from_env(LOG_ENV_VAR));
    tracing_subscriber::registry()
        .with(env_filter)
        .try_init()
        .is_ok()
}
