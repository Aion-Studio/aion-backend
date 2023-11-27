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
    pub mod quest;
    pub mod region;
    pub mod resources;
}

mod repos {
    // pub mod game_engine_repo;
    // pub mod hero_repo;
    pub mod repo;
    // pub mod action_repo;
    // pub mod resources_repo;
}

mod events {
    pub mod dispatcher;
    pub mod game;
    pub mod handle_channeling;
    pub mod handle_explore;
    pub mod handle_lootbox;
    pub mod handle_quest;
    pub mod initialize;
}

mod services {
    pub mod impls {

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
        pub mod channel;
        pub mod explore;
        pub mod action_names;
        pub mod off_beat_actions;
    }
}

pub mod handlers {
    pub mod heroes;
    pub mod regions;
    pub mod tasks;
    pub mod response;

    pub mod quest;
}

pub mod infra;
pub mod messenger;

mod logger;
#[cfg(test)]
pub mod test_helpers;

const LOG_ENV_VAR: &str = "INDEXER_LOG";
#[cfg(not(all(tokio_unstable, feature = "debug")))]
pub fn tracing_subscribe() -> bool {
    use std::env::{set_var, var};

    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    if var("INDEXER_LOG").is_err() {
        set_var("INDEXER_LOG", "debug,tokio=warn,prisma=info,quaint=info");
    }

    println!("Tracing subscriber initialized");
    let env_filter = fmt::layer().with_filter(EnvFilter::from_env(LOG_ENV_VAR));
    tracing_subscriber::registry()
        .with(env_filter)
        .try_init()
        .is_ok()
}
