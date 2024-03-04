use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use prisma_client_rust::chrono;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use tokio::join;
use tokio::sync::{mpsc, Mutex, Notify, oneshot};
use tokio::sync::mpsc::Sender;
use tokio::time::timeout;
use tracing::error;
use tracing::log::info;

use crate::{
    events::combat::{CombatEncounter, CombatTurnMessage},
    models::talent::Talent,
    services::{tasks::action_names::Responder, traits::combat_decision_maker::DecisionMaker},
};
use crate::events::combat::CombatTurnMessage::{PlayerTurn, YourTurn};
use crate::models::hero::Hero;
use crate::models::npc::{CpuCombatantDecisionMaker, Monster};

#[derive(Debug)]
pub enum ControllerMessage {
    RemoveEncounter {
        encounter_id: String,
    },
    AddEncounter {
        encounter: CombatEncounter,
    },
    AddDecisionMaker {
        participant_id: String,
        decision_maker: Arc<Mutex<dyn DecisionMaker + Send + Sync>>,
    },
    StartEncounter {
        encounter_id: String,
    },
    NotifyPlayers {
        message: CombatTurnMessage,
        sender: (String, Sender<CombatTurnMessage>),
    },
    RequestState {
        combatant_id: String,
        tx: oneshot::Sender<Option<CombatTurnMessage>>,
    },
    CreateNpcEncounter {
        hero: Hero,
        npc: Monster,
    },
    Combat((CombatCommand, String, Responder<()>)), // Add other messages as necessary
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
}

pub struct CombatController {
    encounters: HashMap<String, Arc<Mutex<CombatEncounter>>>,
    message_sender: Sender<ControllerMessage>, // New, for internal use
    decision_makers: HashMap<String, Arc<Mutex<dyn DecisionMaker + Send + Sync>>>,
    result_senders: HashMap<String, Sender<CombatTurnMessage>>, // decision_maker_id / combatant_id to result sender
    shutdown_signals: HashMap<String, Arc<Notify>>, // decision_maker_id / combatant_id to shutdown signal
}

impl CombatController {
    pub fn new(message_sender: Sender<ControllerMessage>) -> Self {
        CombatController {
            encounters: HashMap::new(),
            message_sender,
            decision_makers: HashMap::new(),
            result_senders: HashMap::new(),
            shutdown_signals: HashMap::new(),
        }
    }

    // This function now returns the encounter's ID instead of a reference
    async fn encounter_id_by_combatant(&self, combatant_id: &str) -> Option<String> {
        for (id, encounter) in self.encounters.iter() {
            let encounter_lock = encounter.lock().await;
            if encounter_lock.has_combatant(combatant_id) {
                return Some(id.clone());
            }
        }
        None
    }

    async fn encounter_by_combatant_id(
        &self,
        combatant_id: &str,
    ) -> Option<Arc<Mutex<CombatEncounter>>> {
        for (_, encounter) in self.encounters.iter() {
            let encounter_lock = encounter.lock().await;
            if encounter_lock.has_combatant(combatant_id) {
                return Some(Arc::clone(encounter));
            }
        }
        None
    }
    async fn start_encounter_for_combatant(&mut self, combatant_id: &str) {
        let shutdown_signal = Arc::new(Notify::new());
        self.shutdown_signals
            .insert(combatant_id.to_string(), shutdown_signal.clone());

        if let Some(decision_maker) = self.decision_makers.get(&combatant_id.to_string()) {
            let (command_sender, mut command_receiver) = mpsc::channel(10);
            let idx = self
                .encounter_by_combatant_id(combatant_id)
                .await
                .unwrap()
                .lock()
                .await
                .get_combatant_idx(combatant_id)
                .unwrap();
            let mut decision_maker_guard = decision_maker.lock().await;
            let result_sender = decision_maker_guard.start(command_sender.clone(), idx);
            self.result_senders
                .insert(combatant_id.to_string(), result_sender.clone());

            let controller_sender = self.message_sender.clone();
            let encounter_clone =
                Arc::clone(&self.encounter_by_combatant_id(combatant_id).await.unwrap());

            let combatant_id = combatant_id.to_string();
            tokio::spawn(async move {
                tokio::select! {
                    _ = shutdown_signal.notified() => {
                        info!("Shutting down listener for combatant: {}", combatant_id);
                    },
                    _ = async {
                        while let Some(command) = command_receiver.recv().await {
                           let sender = result_sender.clone();

                            let mut encounter = encounter_clone.lock().await;
                            info!("Executing turn for combatant: {:?}", encounter.whos_turn());
                            match encounter.process_combat_turn(command, &combatant_id) {
                                Ok(result) => {
                                    controller_sender.send(ControllerMessage::NotifyPlayers { message: result.clone(), sender: (combatant_id.to_string(), sender.clone()) }).await.unwrap();

                                    if !matches!(result, CombatTurnMessage::Winner(_)) {
                                        info!("Notifying players of next turn, combatant {:?} is up next", encounter.whos_turn());
                                        controller_sender.clone().send(ControllerMessage::NotifyPlayers {
                                            message: PlayerTurn(encounter.whos_turn()), sender: (combatant_id.to_string(), sender)
                                        }).await.unwrap();
                                    } else {
                                        info!("battle done, sending winner message");
                                        let encounter_id = encounter.get_id().clone();
                                        // drop(encounter); // Drop the lock before sending the message.
                                        let controller_sender = controller_sender.clone();
                                            controller_sender.clone()
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

    pub async fn run(&mut self, mut message_receiver: mpsc::Receiver<ControllerMessage>) {
        while let Some(message) = message_receiver.recv().await {
            info!("Received message: {:?}", message);
            let sender = self.message_sender.clone();

            match message {
                ControllerMessage::RemoveEncounter { encounter_id } => {
                    self.encounters.remove(&encounter_id);
                }
                ControllerMessage::Combat((command, from_id, resp)) => match command {
                    CombatCommand::EnterBattle(maybe_decision_maker) => {
                        if let Some(decision_maker) = maybe_decision_maker {
                            self.add_decision_maker(from_id.clone(), decision_maker);
                        }
                        let opponent_id: String;
                        let mut npc_dec_maker_started = true;
                        {
                            let encounter = self.encounter_by_combatant_id(&from_id).await.unwrap();
                            let encounter_guard = encounter.lock().await;
                            let opponent_guard = encounter_guard.get_opponent(&from_id);
                            let opponent = opponent_guard.lock().unwrap();
                            opponent_id = {
                                opponent.get_id().clone() // Example of cloning data out
                            };
                            if let Some(npc) = opponent.as_any().downcast_ref::<Monster>() {
                                let needs_decision_maker =
                                    self.decision_makers.get(&opponent_id).is_none();
                                if needs_decision_maker {
                                    // Construct the decision maker outside of the async block
                                    let npc_decision_maker = Arc::new(Mutex::new(
                                        CpuCombatantDecisionMaker::new(npc.clone()),
                                    ));
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
                    }

                    CombatCommand::Attack => {
                        info!("Attacking");
                    }
                    CombatCommand::UseTalent(opponent_id, talent) => {}
                },

                ControllerMessage::RemoveDecisionMakers { encounter_id, resp } => {
                    // Perform cleanup logic here...
                    info!("RemoveDecisionMakers hit");
                    let encounter = self.encounters.get(&encounter_id).unwrap();
                    let combatant_ids = encounter.lock().await.get_combatant_ids();
                    // shuts down listeners to each decision maker
                    for combatant_id in combatant_ids {
                        self.message_sender
                            .send(ControllerMessage::RemoveSingleDecisionMaker { combatant_id })
                            .await
                            .unwrap();
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
                            Ok(x) => {
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
                    let encounter_id = encounter.get_id();
                    self.encounters
                        .insert(encounter_id, Arc::new(Mutex::new(encounter)));
                }
                ControllerMessage::CreateNpcEncounter { hero, npc } => {
                    let encounter = CombatEncounter::new(hero, npc);
                    self.add_encounter(encounter);
                }
                ControllerMessage::AddDecisionMaker {
                    participant_id,
                    decision_maker,
                } => {
                    self.decision_makers.insert(participant_id, decision_maker);
                }
                ControllerMessage::StartEncounter { encounter_id } => {
                    if let Some(encounter) = self.encounters.get(&encounter_id) {
                        // Start the encounter, similar to your existing start_encounter logic
                    }
                }
                ControllerMessage::RequestState { combatant_id, tx } => {
                    let encounter_id = self.encounter_id_by_combatant(&combatant_id).await;
                    match encounter_id {
                        Some(id) => {
                            let encounter = self.encounters.get(&id).unwrap();
                            let cmd = CombatTurnMessage::EncounterState(
                                encounter.lock().await.clone(),
                                combatant_id.clone(),
                            );
                            tx.send(Some(cmd)).unwrap();
                        }
                        None => {
                            info!("Could not find encounter for combatant: {:?}", combatant_id);
                            tx.send(None).unwrap();
                        }
                    }
                }
                ControllerMessage::NotifyPlayers {
                    message,
                    sender: (id, sender),
                } => {
                    let message = message.clone();
                    let opponent = self
                        .encounter_by_combatant_id(&id)
                        .await
                        .unwrap()
                        .lock()
                        .await
                        .get_opponent(&id);
                    let opponent_sender =
                        self.result_senders.get(&opponent.lock().unwrap().get_id());
                    let message = match message {
                        PlayerTurn(idx) => {
                            let encounter = self.encounter_by_combatant_id(&id).await.unwrap();
                            let encounter = encounter.lock().await;
                            let turn_idx = encounter.get_combatant_idx(&id);
                            match turn_idx {
                                Some(turn_idx) => {
                                    if turn_idx == idx {
                                        let combatant = encounter
                                            .get_combatant(turn_idx, None)
                                            .lock()
                                            .unwrap()
                                            .clone_box();
                                        YourTurn(combatant)
                                    } else {
                                        PlayerTurn(idx)
                                    }
                                }
                                None => PlayerTurn(idx),
                            }
                        }
                        _ => message.clone(),
                    };
                    let (_, _) = join!(
                        opponent_sender.unwrap().send(message.clone()),
                        sender.send(message.clone())
                    );
                }
            }
        }
    }

    pub fn add_encounter(&mut self, encounter: CombatEncounter) {
        self.encounters
            .insert(encounter.get_id(), Arc::new(Mutex::new(encounter)));
    }
    pub fn add_decision_maker(
        &mut self,
        participant_id: String,
        decision_maker: Arc<Mutex<dyn DecisionMaker + Send + Sync>>,
    ) {
        self.decision_makers.insert(participant_id, decision_maker);
    }
}

#[derive(Debug)]
pub enum CombatCommand {
    EnterBattle(Option<Arc<Mutex<dyn DecisionMaker + Send + Sync>>>),
    Attack,
    UseTalent(String, Talent), // Use a talent: (Talent)
}

impl Serialize for CombatCommand {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CombatCommand::EnterBattle(_) => serializer.serialize_str("EnterBattle"),
            CombatCommand::Attack => serializer.serialize_str("Attack"),
            CombatCommand::UseTalent(target_id, talent) => {
                serializer.serialize_str(&format!("UseTalent({}, {:?})", target_id, talent))
            }
        }
    }
}

impl<'de> Deserialize<'de> for CombatCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommandVisitor;

        impl<'de> Visitor<'de> for CommandVisitor {
            type Value = CombatCommand;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a combat command")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "EnterBattle" => Ok(CombatCommand::EnterBattle(None)),
                    "Attack" => Ok(CombatCommand::Attack),
                    // ... (Handle UseTalent similar to your existing code)
                    _ => Err(de::Error::custom("Unknown command")),
                }
            }
        }

        deserializer.deserialize_str(CommandVisitor)
    }
}
