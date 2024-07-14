use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::sync::{Mutex, RwLock};
use tracing::error;

use crate::services::impls::redis_storage::RedisStorage;

use super::combat::CombatEncounter;

pub struct PersistentCombatEncounter {
    encounter: Arc<RwLock<CombatEncounter>>,
    storage: Arc<RedisStorage>,
}

impl PersistentCombatEncounter {
    pub fn new(encounter: CombatEncounter, storage: RedisStorage) -> Self {
        Self {
            encounter: Arc::new(RwLock::new(encounter)),
            storage: Arc::new(storage),
        }
    }

    pub async fn read(&self) -> tokio::sync::RwLockReadGuard<'_, CombatEncounter> {
        self.encounter.read().await
    }

    pub async fn write(&self) -> impl DerefMut<Target = CombatEncounter> + '_ {
        let guard = self.encounter.write().await;
        let storage = self.storage.clone();
        let encounter = self.encounter.clone();

        tokio::spawn(async move {
            let enc = encounter.read().await;
            if let Err(e) = storage.store_encounter(&enc).await {
                error!("Failed to persist encounter: {:?}", e);
            }
        });

        guard
    }
}
