use std::{collections::HashMap, sync::Arc};

use tokio::sync::{
    mpsc::{self, Sender},
    Mutex, Notify,
};

use crate::{
    events::{
        combat::{CombatEncounter, CombatTurnMessage},
        persistant_wrapper::PersistentCombatEncounter,
    },
    models::combatant::CombatantType,
    services::traits::combat_decision_maker::DecisionMaker,
};

use super::redis_storage::RedisStorage;

pub struct SharedState {
    storage: Arc<Mutex<RedisStorage>>,
    pub decision_makers: HashMap<String, Arc<Mutex<dyn DecisionMaker + Send + Sync>>>,
    result_senders: HashMap<String, Sender<CombatTurnMessage>>,
    shutdown_signals: HashMap<String, Arc<Notify>>,
}

impl SharedState {
    pub fn new(redis_uri: &str) -> Self {
        Self {
            storage: Arc::new(Mutex::new(RedisStorage::new(redis_uri))),
            decision_makers: HashMap::new(),
            result_senders: HashMap::new(),
            shutdown_signals: HashMap::new(),
        }
    }

    pub async fn get_encounter(&self, encounter_id: &str) -> Option<PersistentCombatEncounter> {
        let storage = self.storage.lock().await;
        if let Some(encounter) = storage.get_encounter(encounter_id).await {
            Some(PersistentCombatEncounter::new(
                encounter,
                self.storage.clone(),
            ))
        } else {
            None
        }
    }

    pub async fn get_encounter_by_combatant_id(
        &self,
        combatant_id: &str,
    ) -> Option<PersistentCombatEncounter> {
        let storage = self.storage.lock().await;
        if let Some(encounter) = storage.get_encounter_by_combatant(combatant_id).await {
            Some(PersistentCombatEncounter::new(
                encounter,
                self.storage.clone(),
            ))
        } else {
            None
        }
    }

    pub async fn set_encounter(
        &self,
        encounter: &CombatEncounter,
    ) -> Result<(), redis::RedisError> {
        let storage = self.storage.lock().await;
        storage.store_encounter(encounter).await
    }
    pub async fn remove_encounter(&self, encounter_id: &str) -> Result<(), redis::RedisError> {
        let storage = self.storage.lock().await;
        storage.remove_encounter(encounter_id).await
    }

    pub async fn encounter_by_combatant_id(&self, combatant_id: &str) -> Option<CombatEncounter> {
        let storage = self.storage.lock().await;
        storage.get_encounter_by_combatant(combatant_id).await
    }

    pub async fn get_combatant_by_encounter(
        &self,
        encounter_id: &str,
        combatant_id: &str,
    ) -> Option<CombatantType> {
        let storage = self.storage.lock().await;
        let enc = storage.get_encounter(encounter_id).await;

        if let Some(mut encounter) = enc {
            let combatant = encounter.get_combatant_by_id(combatant_id);
            combatant.cloned()
        } else {
            None
        }
    }

    pub fn add_decision_maker(
        &mut self,
        participant_id: String,
        decision_maker: Arc<Mutex<dyn DecisionMaker + Send + Sync>>,
    ) {
        self.decision_makers.insert(participant_id, decision_maker);
    }

    pub fn remove_decision_maker(&mut self, participant_id: &str) {
        self.decision_makers.remove(participant_id);
    }

    pub async fn has_decision_maker(&self, participant_id: &str) -> bool {
        self.decision_makers.contains_key(participant_id)
    }

    pub fn add_result_sender(&mut self, participant_id: String, sender: Sender<CombatTurnMessage>) {
        self.result_senders.insert(participant_id, sender);
    }

    pub fn get_result_sender(&self, participant_id: &str) -> Option<&Sender<CombatTurnMessage>> {
        self.result_senders.get(participant_id)
    }

    pub fn remove_result_sender(&mut self, participant_id: &str) {
        self.result_senders.remove(participant_id);
    }

    pub fn add_shutdown_signal(&mut self, participant_id: String, signal: Arc<Notify>) {
        self.shutdown_signals.insert(participant_id, signal);
    }

    pub fn remove_shutdown_signal(&mut self, participant_id: &str) -> Option<Arc<Notify>> {
        self.shutdown_signals.remove(participant_id)
    }
}
