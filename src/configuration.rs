use config::{Config, ConfigError};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

use prisma_client_rust::chrono::Duration;
use secrecy::Secret;
use serde::Deserialize;
use tracing::{error, info};

use crate::models::region::{leyline_map, RegionName};

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub redis_uri: String,
    pub hmac_secret_key: Secret<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub url: String,
    pub name: String,
    pub params: String,
}

#[derive(Deserialize, Clone)]
pub struct RedisSettings {
    pub url: Secret<String>,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    check_environment();

    if env::var("APP_ENVIRONMENT").eq(&Ok("local".to_string())) {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("configuration");

        // Detect the running environment.
        // Default to `local` if unspecified.
        // let environment: Environment = std::env::var("APP_ENVIRONMENT")
        //     .unwrap_or_else(|_| "local".into())
        //     .try_into()
        //     .expect("Failed to parse APP_ENVIRONMENT.");
        // let environment_filename = format!("{}.yaml", environment.as_str());
        let settings = config::Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            // .add_source(config::File::from(
            //     configuration_directory.join(environment_filename),
            // ))
            // Add in settings from environment variables (with a prefix of APP and '__' as separator)
            // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        settings.try_deserialize::<Settings>()
    } else {
        info!("Connecting to cloud db.....");
        dotenv().ok();

        let config = Config::builder()
            .add_source(
                config::Environment::default()
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()
            .unwrap();

        let res: Result<Settings, ConfigError> = config.try_deserialize();
        res
    }
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

// region name to duration
#[derive(Clone, Debug)]
pub struct ExploreDurations(pub HashMap<RegionName, Duration>);

#[allow(deprecated)]
impl ExploreDurations {
    pub fn get_durations() -> Self {
        let mut durations = HashMap::new();
        durations.insert(RegionName::Dusane, Duration::seconds(0));
        durations.insert(RegionName::Buzna, Duration::seconds(0));
        Self(durations)
    }
}
// leyline to duration
#[derive(Clone, Debug)]
pub struct ChannelDurations(pub HashMap<String, Duration>);
#[allow(deprecated)]
impl ChannelDurations {
    pub fn get_durations() -> Self {
        let mut durations = HashMap::new();

        let all_leylines = leyline_map();
        for (name, _) in all_leylines.iter() {
            durations.insert(name.clone(), Duration::seconds(10));
        }
        Self(durations)
    }
}

#[derive(Clone, Debug)]
pub enum DurationType {
    Explore(ExploreDurations),
    Channel(ChannelDurations),
}

pub fn get_durations() -> HashMap<String, DurationType> {
    let mut durations = HashMap::new();
    durations.insert(
        "Explore".to_string(),
        DurationType::Explore(ExploreDurations::get_durations()),
    );
    durations.insert(
        "Channel".to_string(),
        DurationType::Channel(ChannelDurations::get_durations()),
    );
    durations
}

pub fn get_explore_durations() -> ExploreDurations {
    let durations = get_durations();
    match durations.get("Explore") {
        Some(DurationType::Explore(durations)) => durations.clone(),
        _ => panic!("No explore durations found"),
    }
}

pub fn check_environment() {
    let required_vars = vec![
        "SUPABASE_API_KEY",
        "SUPABASE_PROJECT_ID",
        "APP_ENVIRONMENT",
        // Add other environment variable names here
    ];

    let mut all_vars_present = true;

    for var in required_vars {
        match env::var(var) {
            Ok(_) => {}
            Err(_) => {
                error!("Error: {} is not set", var);
                all_vars_present = false;
            }
        }
    }

    if !all_vars_present {
        eprintln!("One or more required environment variables are missing.");
        std::process::exit(1);
    }
}
