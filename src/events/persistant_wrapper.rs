use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::{Mutex, RwLock};
use tracing::error;

use crate::services::impls::redis_storage::RedisStorage;

use super::combat::CombatEncounter;

#[derive(Debug)]
pub struct PersistentCombatEncounter {
    encounter: CombatEncounter,
    storage: Arc<Mutex<RedisStorage>>,
}

impl PersistentCombatEncounter {
    pub fn new(encounter: CombatEncounter, storage: Arc<Mutex<RedisStorage>>) -> Self {
        Self { encounter, storage }
    }

    pub fn get(&self) -> &CombatEncounter {
        &self.encounter
    }

    pub fn get_mut(&mut self) -> &mut CombatEncounter {
        &mut self.encounter
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let mut storage = self.storage.lock().await;
        storage.store_encounter(&self.encounter).await?;
        Ok(())
    }

    pub async fn modify<F, R>(&mut self, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&mut CombatEncounter) -> R,
    {
        let result = f(&mut self.encounter);
        self.save().await?;
        Ok(result)
    }
}

impl Drop for PersistentCombatEncounter {
    fn drop(&mut self) {
        let storage = self.storage.clone();
        let encounter = self.encounter.clone();
        tokio::spawn(async move {
            if let Ok(mut storage_lock) = storage.try_lock() {
                if let Err(e) = storage_lock.store_encounter(&encounter).await {
                    error!("Failed to persist encounter on drop: {:?}", e);
                }
            } else {
                error!("Failed to acquire storage lock for persisting encounter on drop");
            }
        });
    }
}
