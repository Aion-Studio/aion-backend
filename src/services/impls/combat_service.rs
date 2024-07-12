use std::collections::HashMap;
use std::sync::{Arc, Once, ONCE_INIT};

use prisma_client_rust::chrono;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::join;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot, Mutex, Notify};
use tracing::error;
use tracing::log::info;

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

use super::redis_storage::RedisStorage;

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
        participant_id: String,
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
    SendMsgsToPlayer {
        combatant_id: String,
        result: CombatTurnMessage,
    },
}

pub struct CombatController {
    storage: RedisStorage,
    message_sender: Sender<ControllerMessage>, // New, for internal use
    decision_makers: HashMap<String, Arc<Mutex<dyn DecisionMaker + Send + Sync>>>,
    result_senders: HashMap<String, Sender<CombatTurnMessage>>, // decision_maker_id / combatant_id to result sender
    shutdown_signals: HashMap<String, Arc<Notify>>, // decision_maker_id / combatant_id to shutdown signal
}

impl CombatController {
    pub fn new(message_sender: Sender<ControllerMessage>, redis_uri: &str) -> Self {
        CombatController {
            storage: RedisStorage::new(redis_uri),
            message_sender,
            decision_makers: HashMap::new(),
            result_senders: HashMap::new(),
            shutdown_signals: HashMap::new(),
        }
    }

    pub async fn get_combatant_by_encounter(
        &self,
        encounter_id: &str,
        combatant_id: &str,
    ) -> Option<CombatantType> {
        let enc = self.storage.get_encounter(encounter_id).await;

        if let Some(mut encounter) = enc {
            let combatant = encounter.get_combatant_by_id(combatant_id);
            if let Some(combatant) = combatant {
                Some(combatant.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    async fn initialize_encounter(&self, encounter_id: &str) -> anyhow::Result<()> {
        let encounter = self
            .get_encounter(encounter_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("Encounter not found: {}", encounter_id))?;

        encounter.write().await.initialize();

        Ok(())
    }

    async fn get_encounter(&self, encounter_id: &str) -> Option<PersistentCombatEncounter> {
        self.storage
            .get_encounter(encounter_id)
            .await
            .map(|encounter| PersistentCombatEncounter::new(encounter, (self.storage).clone()))
    }

    async fn set_encounter(&self, encounter: &CombatEncounter) -> Result<(), redis::RedisError> {
        self.storage.store_encounter(encounter).await
    }

    async fn remove_encounter(&self, encounter_id: &str) -> Result<(), redis::RedisError> {
        self.storage.remove_encounter(encounter_id).await
    }

    async fn encounter_by_combatant_id(&self, combatant_id: &str) -> Option<CombatEncounter> {
        self.storage.get_encounter_by_combatant(combatant_id).await
    }

    async fn start_encounter_for_combatant(&mut self, combatant_id: &str) {
        println!("Starting encounter for combatant: {}", combatant_id);
        let shutdown_signal = Arc::new(Notify::new());
        self.shutdown_signals
            .insert(combatant_id.to_string(), shutdown_signal.clone());

        if let Some(decision_maker) = self.decision_makers.get(&combatant_id.to_string()) {
            println!("finds decision maker");
            let (command_sender, mut command_receiver) = mpsc::channel(10);
            let encounter = self.encounter_by_combatant_id(combatant_id).await.unwrap();
            let idx = encounter.get_combatant_idx(combatant_id).unwrap();

            let mut decision_maker_guard = decision_maker.lock().await;
            //
            // Start Decision Maker listening
            let result_sender = decision_maker_guard.start(command_sender.clone(), idx);
            //
            //
            self.result_senders
                .insert(combatant_id.to_string(), result_sender.clone());

            let controller_sender = self.message_sender.clone();
            let storage = self.storage.clone();

            let combatant_id = combatant_id.to_string();

            tokio::spawn(async move {
                tokio::select! {
                    _ = shutdown_signal.notified() => {
                        info!("Shutting down listener for combatant: {}", combatant_id);
                    },
                    _ = async {
                        while let Some(command) = command_receiver.recv().await {
                            let mut encounter = storage.get_encounter_by_combatant(&combatant_id).await.unwrap();

                             println!("waiting for ocmbat turn");
                            match encounter.process_combat_turn(command, &combatant_id) {
                                Ok(result) => {
                                    // Persist the updated encounter
                                    if let Err(e) = storage.store_encounter(&encounter).await {
                                        error!("Failed to persist encounter: {:?}", e);
                                    }

                                    if !matches!(result, CombatTurnMessage::Winner(_)) {
                                        controller_sender.send(ControllerMessage::SendMsgsToPlayer {
                                            combatant_id: combatant_id.to_string(),
                                            result: result.clone()
                                        }).await.unwrap();
                                    } else {
                                        info!("Battle is over, we got a winner");
                                        let encounter_id = encounter.get_id().clone();
                                        controller_sender
                                            .try_send(ControllerMessage::CleanupEncounter { encounter_id })
                                            .expect("Failed to send cleanup message");
                                    }
                                }
                                Err(e) => {
                                    error!("Error processing combat turn: {:?}", e);
                                }
                            }
                        }
                    } => {}
                }
            });
        }
    }

    #[allow(dead_code)]
    async fn update_players_with_msgs(
        &self,
        encounter_state: EncounterState,
        combatant_id: &str,
        result: &CombatTurnMessage,
    ) {
        println!(
            "Updating players with messages {:?} {:?} {:?}",
            encounter_state, combatant_id, result
        );
    }

    async fn construct_player_state(
        &self,
        combatant_id: &str,
        encounter: &mut CombatEncounter,
    ) -> CombatantState {
        let player = encounter.get_combatant_by_id(combatant_id).unwrap();
        let state = player.as_combatant().get_player_state();
        state
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

    pub async fn run(&mut self, mut message_receiver: mpsc::Receiver<ControllerMessage>) {
        while let Some(message) = message_receiver.recv().await {
            let sender = self.message_sender.clone();

            match message {
                ControllerMessage::RemoveEncounter { encounter_id } => {
                    if let Err(e) = self.remove_encounter(&encounter_id).await {
                        error!("Error removing encounter: {:?}", e);
                    }

                    info!("Removed encounter: {}", encounter_id);
                }
                ControllerMessage::GetCombatant {
                    encounter_id,
                    combatant_id,
                    tx,
                } => {
                    let combatant = self
                        .get_combatant_by_encounter(&encounter_id, &combatant_id)
                        .await;
                    tx.send(combatant).unwrap();
                }
                ControllerMessage::Combat((command, from_id)) => match command {
                    CombatCommand::EnterBattle(battle_data) => {
                        if let Some(decision_maker) = battle_data.0 {
                            self.add_decision_maker(from_id.clone(), decision_maker);
                            let opponent_id: String;
                            let mut npc_dec_maker_started = true;
                            {
                                let mut encounter =
                                    self.encounter_by_combatant_id(&from_id).await.unwrap();
                                let opponent_guard = encounter.get_opponent(&from_id);
                                let opponent = opponent_guard.as_combatant();
                                opponent_id = {
                                    opponent.get_id().clone() // Example of cloning data out
                                };
                                if let Some(npc) = opponent.as_any().downcast_ref::<Monster>() {
                                    let needs_decision_maker =
                                        self.decision_makers.get(&opponent_id).is_none();
                                    if needs_decision_maker {
                                        // Construct the decision maker outside of the async block
                                        let npc_decision_maker =
                                            Arc::new(Mutex::new(CpuCombatantDecisionMaker::new(
                                                npc.clone(),
                                                encounter.get_id(),
                                            )));
                                        // Add the decision maker here
                                        self.add_decision_maker(
                                            opponent_id.clone(),
                                            npc_decision_maker,
                                        );
                                        npc_dec_maker_started = false;
                                    }
                                }
                            }
                            if !npc_dec_maker_started {
                                // Assuming we can start the encounter without further mutating `self` directly
                                self.start_encounter_for_combatant(&opponent_id).await;
                            }
                            self.start_encounter_for_combatant(&from_id).await;
                            let mut encounter =
                                self.encounter_by_combatant_id(&from_id).await.unwrap();

                            self.initialize_encounter(&encounter.get_id())
                                .await
                                .expect("Failed to initialize encounter");
                        }
                    }
                    CombatCommand::LeaveBattle => {
                        if let Some(encounter) = self.encounter_by_combatant_id(&from_id).await {
                            for combatant_id in encounter.get_combatant_ids() {
                                self.message_sender
                                    .send(ControllerMessage::RemoveSingleDecisionMaker {
                                        combatant_id,
                                    })
                                    .await
                                    .unwrap();
                            }
                            info!(
                                "removed decision makers for encounter: {:?}",
                                encounter.get_id()
                            );
                            self.message_sender
                                .send(ControllerMessage::RemoveEncounter {
                                    encounter_id: encounter.get_id(),
                                })
                                .await
                                .unwrap();
                        }
                    }

                    CombatCommand::UseSpell(opponent_id, talent) => {
                        println!("UseTalent: {} {:?}", opponent_id, talent);
                    }
                    _ => {}
                },

                ControllerMessage::RemoveDecisionMakers { encounter_id, resp } => {
                    // Perform cleanup logic here...
                    if let Some(encounter) = self.get_encounter(&encounter_id).await {
                        for combatant_id in encounter.read().await.get_combatant_ids() {
                            self.message_sender
                                .send(ControllerMessage::RemoveSingleDecisionMaker { combatant_id })
                                .await
                                .unwrap();
                        }
                    }
                    resp.send(()).unwrap();
                }
                ControllerMessage::RemoveSingleDecisionMaker { combatant_id } => {
                    if let Some(shutdown_signal) = self.shutdown_signals.remove(&combatant_id) {
                        shutdown_signal.notify_waiters();
                        if let Some(decision_maker) = self.decision_makers.get(&combatant_id) {
                            let mut decision_maker_guard = decision_maker.lock().await;
                            decision_maker_guard.shutdown();
                        }
                        self.decision_makers.remove(&combatant_id);
                    }
                }
                ControllerMessage::CleanupEncounter { encounter_id } => {
                    let encounter_id_copy = encounter_id.clone();
                    let (tx, rx) = oneshot::channel();
                    let sender = sender.clone();
                    let cloned = sender.clone();
                    tokio::spawn(async move {
                        let sender = cloned.clone();
                        tokio::spawn(async move {
                            let send_future =
                                sender
                                    .clone()
                                    .try_send(ControllerMessage::RemoveDecisionMakers {
                                        encounter_id: encounter_id_copy,
                                        resp: tx,
                                    });
                            match send_future {
                                Ok(_) => {
                                    tracing::info!(
                                        "{} - RemoveDecisionMakers message sent for ",
                                        chrono::Utc::now()
                                    );
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Failed to send RemoveDecisionMakers message: {:?}",
                                        e
                                    )
                                }
                            }

                            tracing::info!("{} - all done with match arm  ", chrono::Utc::now());
                        });
                    });

                    let sender = sender.clone();
                    tokio::spawn(async move {
                        match rx.await {
                            Ok(_) => {
                                sender
                                    .try_send(ControllerMessage::RemoveEncounter {
                                        encounter_id: encounter_id.clone(),
                                    })
                                    .unwrap();
                            }
                            Err(e) => {
                                tracing::error!("Error cleaning up encounter: {:?}", e);
                            }
                        }
                    });
                }
                ControllerMessage::AddEncounter { encounter } => {
                    if let Err(e) = self.set_encounter(&encounter).await {
                        error!("Failed to add encounter: {:?}", e);
                    }
                }

                ControllerMessage::EncounterCheck { combatant_id, tx } => {
                    let encounter = self.encounter_by_combatant_id(&combatant_id).await;
                    tx.send(encounter.is_some()).unwrap();
                }
                ControllerMessage::CreateNpcEncounter {
                    hero,
                    npc,
                    action_id,
                } => {
                    let mut encounter = CombatEncounter::new(hero.to_combatant(), npc);
                    encounter.set_action_id(action_id);
                    if let Err(e) = self.set_encounter(&encounter).await {
                        error!("Failed to create NPC encounter: {:?}", e);
                    }
                }
                ControllerMessage::AddDecisionMaker {
                    participant_id,
                    decision_maker,
                } => {
                    self.decision_makers.insert(participant_id, decision_maker);
                }

                ControllerMessage::RequestState { combatant_id, tx } => {
                    if let Some(mut encounter) = self.encounter_by_combatant_id(&combatant_id).await
                    {
                        info!("found encounter for combatant: {}", combatant_id);
                        self.initialize_encounter(&encounter.get_id())
                            .await
                            .expect("Failed to initialize encounter");

                        // Persist the initialized encounter
                        if let Err(e) = self.set_encounter(&encounter).await {
                            error!("Failed to persist initialized encounter: {:?}", e);
                        }

                        let action_id = encounter.action_id.clone();
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
                        let encounter_state = EncounterState {
                            player_state,
                            npc_state,
                            turn: encounter.whos_turn(),
                            round: encounter.round,
                        };
                        tx.send((
                            Some(CombatTurnMessage::EncounterData(encounter_state)),
                            action_id,
                        ))
                        .unwrap();
                    } else {
                        info!("No encounter found for combatant: {}", combatant_id);
                        tx.send((None, None)).unwrap();
                    }
                }
                ControllerMessage::NotifyPlayers {
                    message,
                    sender: (id, sender),
                } => {
                    let mut encounter = self.encounter_by_combatant_id(&id).await.unwrap();

                    let opponent_id = encounter.get_opponent(&id).as_combatant().get_id();
                    let opponent_sender = self.result_senders.get(&opponent_id);
                    info!("Sending message to players: {:?}", message);
                    let (_, _) = join!(
                        opponent_sender.unwrap().send(message.clone()),
                        sender.send(message)
                    );
                }
                // Use this to construct as many msgs to send to client based on the result of a combat turn
                ControllerMessage::SendMsgsToPlayer {
                    combatant_id,
                    result,
                } => {
                    let sender = self.result_senders.get(&combatant_id).unwrap();
                    let controller_sender = self.message_sender.clone();

                    controller_sender
                        .clone()
                        .send(ControllerMessage::NotifyPlayers {
                            message: result,
                            sender: (combatant_id.to_string(), sender.clone()),
                        })
                        .await
                        .unwrap();
                    let encounter = self.encounter_state(combatant_id.clone()).await.unwrap();

                    controller_sender
                        .send(ControllerMessage::NotifyPlayers {
                            message: CombatTurnMessage::EncounterData(encounter),
                            sender: (combatant_id.to_string(), sender.clone()),
                        })
                        .await
                        .unwrap();
                }
            }
        }
    }

    pub fn add_decision_maker(
        &mut self,
        participant_id: String,
        decision_maker: Arc<Mutex<dyn DecisionMaker + Send + Sync>>,
    ) {
        self.decision_makers.insert(participant_id, decision_maker);
    }
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
