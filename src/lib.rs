// pub mod authentication;
#![recursion_limit = "256"]
#![allow(deprecated)]
pub mod configuration;
// pub mod idempotency;
// pub mod routes;
// pub mod session_state;
pub mod types;
pub mod utils;
pub mod webserver;

pub mod fp_macros;
// pub mod storable;
mod db_client;

pub use db_client::*;

pub mod telemetry;

#[allow(warnings, unused)]
pub mod prisma;
#[allow(dead_code)]
#[allow(unused_variables)]
mod models {
    pub mod card_effect;
    pub mod combatant;
    pub mod date_times;
    pub mod hero;
    pub mod hero_combatant;
    pub mod npc;
    pub mod quest;

    pub mod player_decision_maker;
    pub mod region;
    pub mod resources;
    pub mod talent;

    pub mod cards;
}

mod repos {
    // pub mod game_engine_repo;
    // pub mod hero_repo;
    pub mod cards;
    pub mod helpers;
    pub mod hero;
    pub mod repo;
    // pub mod action_repo;
    // pub mod resources_repo;
}

mod events {
    pub mod combat;
    pub mod game;
    pub mod handle_channeling;
    pub mod handle_costs;
    pub mod handle_explore;
    pub mod handle_lootbox;
    pub mod handle_quest;
    pub mod persistant_wrapper;
}

mod services {
    pub mod impls {
        pub mod combat_service;
        pub mod redis_storage;
        pub mod tasks;
    }

    pub mod traits {
        pub mod async_task;
        pub mod combat_decision_maker;
    }
    pub mod tasks {
        pub mod action_names;
        pub mod channel;
        pub mod explore;
        pub mod off_beat_actions;
    }
}

mod tests {
    #[cfg(test)]
    pub mod helpers;
}

pub mod endpoints {
    pub mod auth;
    pub mod cards;
    pub mod combat_socket;
    pub mod heroes;
    pub mod quest;
    pub mod regions;
    pub mod response;
    pub mod tasks;
}

pub mod authentication;
pub mod infra;
pub mod jsontoken;
pub mod messenger;
pub mod session_state;

mod logger;

const LOG_ENV_VAR: &str = "INDEXER_LOG";
#[cfg(not(all(tokio_unstable, feature = "debug")))]
pub fn tracing_subscribe() -> bool {
    use std::env::{set_var, var};

    use tracing::info;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    if var("INDEXER_LOG").is_err() {
        set_var("INDEXER_LOG", "debug,tokio=warn,prisma=info,quaint=info");
    }

    info!("Tracing subscriber initialized");
    let env_filter = fmt::layer().with_filter(EnvFilter::from_env(LOG_ENV_VAR));
    tracing_subscriber::registry()
        .with(env_filter)
        .try_init()
        .is_ok()
}
