use std::collections::HashMap;

use prisma_client_rust::chrono::Duration;
use secrecy::Secret;
use serde::Deserialize;

use crate::models::region::RegionName;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub redis_uri: Secret<String>,
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
}

#[derive(Deserialize, Clone)]
pub struct RedisSettings {
    pub url: Secret<String>,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment.
    // Default to `local` if unspecified.
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
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

pub type Durations = HashMap<RegionName, Duration>;

pub fn get_region_durations() -> Durations {
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::seconds(3));
    durations.insert(RegionName::Buzna, Duration::seconds(3));
    durations
}
