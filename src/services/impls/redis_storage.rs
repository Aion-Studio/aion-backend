use std::sync::Arc;

use redis::{AsyncCommands, Commands};
use tokio::sync::Mutex;
use tracing::info;

use crate::events::combat::CombatEncounter;

#[derive(Clone, Debug)]
pub struct RedisStorage {
    client: Arc<Mutex<redis::Client>>,
}

impl RedisStorage {
    pub fn new(redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).unwrap_or_else(|e| {
            eprintln!("Failed to create Redis client: {}", e);
            eprintln!("Redis URL: {}", redis_url);
            panic!("Redis client creation failed");
        });
        RedisStorage {
            client: Arc::new(Mutex::new(client)),
        }
    }

    async fn get_connection(&self) -> redis::aio::Connection {
        self.client
            .lock()
            .await
            .get_async_connection()
            .await
            .expect("Failed to get Redis connection")
    }

    pub async fn store_encounter(
        &self,
        encounter: &CombatEncounter,
    ) -> Result<(), redis::RedisError> {
        info!("Storing encounter: {:?}", encounter.get_id());
        let mut conn = self.get_connection().await;
        let encounter_json = serde_json::to_string(encounter).map_err(|e| {
            redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Serialization error",
                e.to_string(),
            ))
        })?;

        // Store the encounter
        conn.set(format!("encounter:{}", encounter.get_id()), encounter_json)
            .await?;

        // Add to the set of all encounters
        conn.sadd("encounters", encounter.get_id()).await?;

        // Create indexes for quick lookup
        for combatant_id in encounter.get_combatant_ids() {
            conn.set(
                format!("combatant_encounter:{}", combatant_id),
                encounter.get_id(),
            )
            .await?;
        }

        Ok(())
    }

    pub async fn get_encounter(&self, encounter_id: &str) -> Option<CombatEncounter> {
        let mut conn = self.get_connection().await;
        let encounter_json: Option<String> =
            conn.get(format!("encounter:{}", encounter_id)).await.ok();
        encounter_json.and_then(|json| serde_json::from_str(&json).ok())
    }

    pub async fn get_encounter_by_combatant(&self, combatant_id: &str) -> Option<CombatEncounter> {
        let mut conn = self.get_connection().await;
        let encounter_id: Option<String> = conn
            .get(format!("combatant_encounter:{}", combatant_id))
            .await
            .ok();
        match encounter_id {
            Some(id) => self.get_encounter(&id).await,
            None => None,
        }
    }

    pub async fn remove_encounter(&self, encounter_id: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.get_connection().await;
        let encounter = self.get_encounter(encounter_id).await;

        if let Some(encounter) = encounter {
            info!("Redis Removing encounter: {:?}", encounter_id);
            // Remove the encounter
            conn.del(format!("encounter:{}", encounter_id)).await?;

            // Remove from the set of all encounters
            conn.srem("encounters", encounter_id).await?;

            // Remove indexes
            for combatant_id in encounter.get_combatant_ids() {
                conn.del(format!("combatant_encounter:{}", combatant_id))
                    .await?;
            }
        }

        Ok(())
    }
}
