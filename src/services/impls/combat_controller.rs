use std::sync::Arc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot, Mutex, Notify};
use tracing::error;

use crate::events::combat::{CombatantState, EncounterState};
use crate::events::persistant_wrapper::PersistentCombatEncounter;
use crate::models::cards::Card;
use crate::models::combatant::CombatantType;
use crate::models::hero::Hero;
use crate::models::npc::{CpuCombatantDecisionMaker, Monster};
use crate::models::talent::Spell;
use crate::{
    events::combat::{CombatEncounter, CombatTurnMessage},
    services::traits::combat_decision_maker::DecisionMaker,
};

use super::combat_messages::MessageHandler;
use super::combat_shared_state::SharedState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatCommand {
    EnterBattle(EnterBattleData),
    LeaveBattle,
    UseSpell(String, Spell), // Use a talent: (Talent)
    PlayCard(Card),
    EndTurn,
}

#[derive(Debug)]
pub enum ControllerMessage {
    EncounterCheck {
        combatant_id: String,
        tx: oneshot::Sender<bool>,
    },
    RemoveEncounter {
        encounter_id: String,
    },
    GetCombatant {
        encounter_id: String,
        combatant_id: String,
        tx: oneshot::Sender<Option<CombatantType>>,
    },
    AddEncounter {
        encounter: CombatEncounter,
    },
    AddDecisionMaker {
        combatant_id: String,
        decision_maker: Arc<Mutex<dyn DecisionMaker + Send + Sync>>,
    },
    NotifyPlayers {
        message: CombatTurnMessage,
        sender: (String, Sender<CombatTurnMessage>),
    },
    RequestState {
        combatant_id: String,
        tx: oneshot::Sender<(Option<CombatTurnMessage>, Option<String>)>,
    },
    CreateNpcEncounter {
        hero: Hero,
        npc: Monster,
        action_id: String,
    },
    Combat((CombatCommand, String)), // Add other messages as necessary
    CleanupEncounter {
        encounter_id: String,
    },
    RemoveDecisionMakers {
        encounter_id: String,
        resp: oneshot::Sender<()>,
    },
    RemoveSingleDecisionMaker {
        combatant_id: String,
    },
    StartEncounterForCombatant {
        combatant_id: String,
    },
    SendMsgsToPlayer {
        combatant_id: String,
        result: CombatTurnMessage,
    },
    GetEncounter {
        combatant_id: String,
        tx: oneshot::Sender<Option<PersistentCombatEncounter>>,
    },
}

impl ControllerMessage {
    pub fn variant_name(&self) -> &str {
        match self {
            ControllerMessage::EncounterCheck { .. } => "EncounterCheck",
            ControllerMessage::RemoveEncounter { .. } => "RemoveEncounter",
            ControllerMessage::GetCombatant { .. } => "GetCombatant",
            ControllerMessage::AddEncounter { .. } => "AddEncounter",
            ControllerMessage::AddDecisionMaker { .. } => "AddDecisionMaker",
            ControllerMessage::NotifyPlayers { .. } => "NotifyPlayers",
            ControllerMessage::RequestState { .. } => "RequestState",
            ControllerMessage::CreateNpcEncounter { .. } => "CreateNpcEncounter",
            ControllerMessage::Combat { .. } => "Combat",
            ControllerMessage::CleanupEncounter { .. } => "CleanupEncounter",
            ControllerMessage::RemoveDecisionMakers { .. } => "RemoveDecisionMakers",
            ControllerMessage::RemoveSingleDecisionMaker { .. } => "RemoveSingleDecisionMaker",
            ControllerMessage::StartEncounterForCombatant { .. } => "StartEncounterForCombatant",
            ControllerMessage::SendMsgsToPlayer { .. } => "SendMsgsToPlayer",
            ControllerMessage::GetEncounter { .. } => "GetEncounter",
        }
    }
}

pub struct CombatController {
    state: Arc<Mutex<SharedState>>,
}

impl CombatController {
    fn new(redis_uri: &str) -> Self {
        Self {
            state: Arc::new(Mutex::new(SharedState::new(redis_uri))),
        }
    }

    pub async fn get_combatant_by_encounter(
        &self,
        encounter_id: &str,
        combatant_id: &str,
    ) -> Option<CombatantType> {
        let state = self.state.lock().await;
        let mut encounter = state.get_encounter(encounter_id).await.unwrap();
        match encounter.get_mut().get_combatant_by_id(combatant_id) {
            Some(combatant) => Some(combatant.clone()),
            None => None,
        }
    }

    pub async fn get_encounter(&self, encounter_id: &str) -> Option<PersistentCombatEncounter> {
        let state = self.state.lock().await;
        state.get_encounter(encounter_id).await
    }

    pub async fn set_encounter(
        &self,
        encounter: &CombatEncounter,
    ) -> Result<(), redis::RedisError> {
        let state = self.state.lock().await;
        let res = state.set_encounter(encounter).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to persist initialized encounter: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn remove_encounter(&self, encounter_id: &str) -> Result<(), redis::RedisError> {
        let state = self.state.lock().await;
        state.remove_encounter(encounter_id).await
    }

    pub async fn encounter_by_combatant_id(&self, combatant_id: &str) -> Option<CombatEncounter> {
        let state = self.state.lock().await;
        state.encounter_by_combatant_id(combatant_id).await
    }

    pub async fn remove_decision_maker(&self, participant_id: &str) {
        let mut state = self.state.lock().await;
        state.remove_decision_maker(participant_id);
    }

    pub async fn add_result_sender(
        &self,
        participant_id: String,
        sender: Sender<CombatTurnMessage>,
    ) {
        self.state
            .lock()
            .await
            .add_result_sender(participant_id, sender);
    }

    pub async fn remove_result_sender(&self, participant_id: &str) {
        self.state.lock().await.remove_result_sender(participant_id);
    }

    pub async fn get_result_sender(
        &self,
        participant_id: &str,
    ) -> Option<Sender<CombatTurnMessage>> {
        self.state
            .lock()
            .await
            .get_result_sender(participant_id)
            .cloned()
    }

    pub async fn add_shutdown_signal(&self, participant_id: String, signal: Arc<Notify>) {
        self.state
            .lock()
            .await
            .add_shutdown_signal(participant_id, signal);
    }

    pub async fn remove_shutdown_signal(&self, participant_id: &str) -> Option<Arc<Notify>> {
        self.state
            .lock()
            .await
            .remove_shutdown_signal(participant_id)
    }

    pub async fn initialize_encounter(&self, encounter_id: &str) -> anyhow::Result<()> {
        if let Some(mut persistent_encounter) = self.get_encounter(encounter_id).await {
            persistent_encounter
                .modify(|encounter| encounter.initialize())
                .await?;
        }
        Ok(())
    }

    pub async fn construct_player_state(
        &self,
        combatant_id: &str,
        encounter: &mut CombatEncounter,
    ) -> CombatantState {
        let player = encounter.get_combatant_by_id(combatant_id).unwrap();
        player.as_combatant().get_player_state()
    }

    pub async fn encounter_state(&self, combatant_id: String) -> Option<EncounterState> {
        if let Some(mut encounter) = self.encounter_by_combatant_id(&combatant_id).await {
            if let Err(e) = self.set_encounter(&encounter).await {
                error!("Failed to persist initialized encounter: {:?}", e);
            }

            let player_state = self
                .construct_player_state(&combatant_id, &mut encounter)
                .await;
            let npc_state = self
                .construct_player_state(
                    &encounter
                        .get_opponent(&combatant_id)
                        .as_combatant()
                        .get_id(),
                    &mut encounter,
                )
                .await;
            Some(EncounterState {
                player_state,
                npc_state,
                turn: encounter.whos_turn(),
                round: encounter.round,
            })
        } else {
            None
        }
    }

    pub async fn process_combat_turn(
        &self,
        combatant_id: &str,
        command: CombatCommand,
    ) -> Result<CombatTurnMessage, anyhow::Error> {
        let mut encounter = self
            .encounter_by_combatant_id(combatant_id)
            .await
            .ok_or_else(|| {
                anyhow::anyhow!("Encounter not found for combatant: {}", combatant_id)
            })?;

        let result = encounter.process_combat_turn(command, combatant_id)?;
        self.set_encounter(&encounter).await?;

        Ok(result)
    }
}

pub fn setup_combat_system(
    redis_uri: &str,
) -> (
    Arc<CombatController>,
    mpsc::Sender<ControllerMessage>,
    MessageHandler,
) {
    let controller = Arc::new(CombatController::new(redis_uri));
    let (tx, rx) = mpsc::channel(100);
    let message_handler =
        MessageHandler::new(controller.state.clone(), controller.clone(), rx, tx.clone());

    (controller, tx, message_handler)
}

#[derive(Debug, Clone)]
pub struct EnterBattleData(pub Option<Arc<Mutex<dyn DecisionMaker + Send + Sync>>>);
// Implement custom serialization for the complex struct
impl Serialize for EnterBattleData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Custom serialization logic here
        serializer.serialize_str("EnterBattle")
    }
}

// Implement custom deserialization for the complex struct
impl<'de> Deserialize<'de> for EnterBattleData {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Custom deserialization logic here
        Ok(EnterBattleData(None))
    }
}
