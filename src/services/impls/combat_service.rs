use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use anyhow::Error;
use futures::future::join_all;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tracing::error;
use tracing::log::info;
use uuid::Uuid;

use crate::endpoints::combat::PlayerDecisionMaker;
use crate::models::combatant::Combatant;
use crate::models::hero::Hero;
use crate::models::npc::{CpuCombatantDecisionMaker, Monster};
use crate::{
    events::combat::{CombatEncounter, CombatTurnMessage},
    models::talent::Talent,
    services::{tasks::action_names::Responder, traits::combat_decision_maker::DecisionMaker},
};

pub type WrappedEncounter = Arc<Mutex<CombatEncounter>>;

type EncounterId = String;
pub enum ControllerMessage {
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
        recipient_ids: Vec<String>,
    },
    RequestState {
        combatant_id: String,
        tx: tokio::sync::oneshot::Sender<CombatTurnMessage>,
    },
    CreateNpcEncounter {
        hero: Hero,
        npc: Monster,
    },
    Combat((CombatCommand, String, Responder<()>)), // Add other messages as necessary
}

pub struct CombatController {
    encounters: HashMap<String, Arc<Mutex<CombatEncounter>>>,
    // Track in-progress encounters
    message_sender: mpsc::Sender<ControllerMessage>, // New, for internal use
    // command_receiver: mpsc::Receiver<(CombatCommand, String, Responder<CombatTurnMessage>)>,
    // Command , attacker_id
    decision_makers: HashMap<String, Arc<Mutex<dyn DecisionMaker + Send + Sync>>>,
}

impl CombatController {
    pub fn new(
        // command_receiver: mpsc::Receiver<(CombatCommand, String, Responder<CombatTurnMessage>)>,
        message_sender: mpsc::Sender<ControllerMessage>,
    ) -> Self {
        CombatController {
            encounters: HashMap::new(),
            // command_receiver,
            message_sender,
            decision_makers: HashMap::new(),
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

    pub async fn notify_players(
        result: CombatTurnMessage,
        result_senders: Vec<Sender<CombatTurnMessage>>,
    ) {
        let result = Arc::new(Mutex::new(result)); // Wrap the result in Arc<Mutex> for shared ownership and mutability
        info!("len of result senders: {}", result_senders.len());
        let futures = result_senders.into_iter().map(|sender| {
            let result_clone = Arc::clone(&result);
            async move {
                let result = result_clone.lock().await;
                sender
                    .send(result.clone())
                    .await
                    .expect("Failed to send combat result");
            }
        });
        join_all(futures).await;
    }

    async fn start_encounter(&mut self, from_id: &str) {
        let encounter = self.encounter_by_combatant_id(&from_id).await.unwrap();
        let combatant_ids = encounter.lock().await.get_combatant_ids();

        let mut result_senders = vec![];
        let mut receivers = vec![];
        for id in combatant_ids.clone() {
            if let Some(decision_maker) = self.decision_makers.get(&id) {
                let (command_sender, command_receiver) = mpsc::channel(10);
                let idx = encounter.lock().await.get_combatant_idx(&id).unwrap();
                let mut decision_maker_guard = decision_maker.lock().await;
                let result_sender = decision_maker_guard.start(command_sender.clone(), idx);
                result_senders.push(result_sender);
                receivers.push((command_receiver, id.clone()));
            }
        }

        for (mut receiver, combatant_id) in receivers {
            let senders = result_senders.clone();
            let encounter_clone = Arc::clone(&encounter);

            tokio::spawn(async move {
                while let Some(command) = receiver.recv().await {
                    let mut encounter = encounter_clone.lock().await;

                    match encounter.process_combat_turn(command, &combatant_id) {
                        Ok(result) => {
                            CombatController::notify_players(result.clone(), senders.clone()).await;
                            if !matches!(result, CombatTurnMessage::Winner(_)) {
                                info!("Notifying players of next turn");
                                CombatController::notify_players(
                                    CombatTurnMessage::PlayerTurn(encounter.whos_turn()),
                                    senders.clone(),
                                )
                                .await;
                            }
                        }
                        Err(e) => {
                            error!("Error processing combat turn: {:?}", e);
                        }
                    }
                }
            });
        }

        //
        // for participant_id in combatant_ids {
        //     if let Some(decision_maker) = self.decision_makers.get(&participant_id) {
        //         let (command_sender, mut command_receiver) = mpsc::channel(10);
        //
        //         let idx = encounter
        //             .lock()
        //             .await
        //             .get_combatant_idx(&participant_id)
        //             .unwrap();
        //
        //         let mut decision_maker_guard = decision_maker.lock().await;
        //
        //         let senders_clone = result_senders.clone();
        //         let encounter_clone = Arc::clone(&encounter);

        //     tokio::spawn(async move {
        //         while let Some(command) = command_receiver.recv().await {
        //             let mut encounter = encounter_clone.lock().await;
        //
        //             match encounter.process_combat_turn(command, &participant_id) {
        //                 Ok(result) => {
        //                     CombatController::notify_players(
        //                         result.clone(),
        //                         senders_clone.clone(),
        //                     )
        //                     .await;
        //                     if !matches!(result, CombatTurnMessage::Winner(_)) {
        //                         info!("Notifying players of next turn");
        //                         CombatController::notify_players(
        //                             CombatTurnMessage::PlayerTurn(encounter.whos_turn()),
        //                             senders_clone.clone(),
        //                         )
        //                         .await;
        //                     }
        //                 }
        //                 Err(e) => {
        //                     error!("Error processing combat turn: {:?}", e);
        //                 }
        //             }
        //         }
        //     });
        // }
    }
    pub async fn run(&mut self, mut message_receiver: mpsc::Receiver<ControllerMessage>) {
        use ControllerMessage::*;
        while let Some(message) = message_receiver.recv().await {
            match message {
                Combat((command, from_id, resp)) => match command {
                    CombatCommand::EnterBattle(maybe_decision_maker) => {
                        if let Some(decision_maker) = maybe_decision_maker {
                            self.add_decision_maker(from_id.clone(), decision_maker);
                        }
                        {
                            let encounter = self.encounter_by_combatant_id(&from_id).await.unwrap();
                            let encounter_guard = encounter.lock().await;
                            let opponent_guard = encounter_guard.get_opponent(&from_id);
                            let opponent = opponent_guard.lock().unwrap();

                            if let Some(npc) = opponent.as_any().downcast_ref::<Monster>() {
                                if !self.decision_makers.contains_key(&opponent.get_id()) {
                                    info!("Adding NPC decision maker for {}", npc.get_id());
                                    let npc_decision_maker = Arc::new(Mutex::new(
                                        CpuCombatantDecisionMaker::new(npc.clone()),
                                    ));
                                    self.add_decision_maker(opponent.get_id(), npc_decision_maker);
                                }
                            }
                        }
                        self.start_encounter(&from_id).await;
                    }
                    CombatCommand::Attack => {
                        info!("Attacking");
                    }
                    CombatCommand::UseTalent(opponent_id, talent) => {}
                },
                AddEncounter { encounter } => {
                    let encounter_id = encounter.get_id();
                    self.encounters
                        .insert(encounter_id, Arc::new(Mutex::new(encounter)));
                }
                CreateNpcEncounter { hero, npc } => {
                    let encounter = CombatEncounter::new(hero, npc);
                    self.add_encounter(encounter);
                }
                AddDecisionMaker {
                    participant_id,
                    decision_maker,
                } => {
                    self.decision_makers.insert(participant_id, decision_maker);
                }
                StartEncounter { encounter_id } => {
                    if let Some(encounter) = self.encounters.get(&encounter_id) {
                        // Start the encounter, similar to your existing start_encounter logic
                    }
                }
                RequestState { combatant_id, tx } => {
                    let encounter_id = self.encounter_id_by_combatant(&combatant_id).await;
                    match encounter_id {
                        Some(id) => {
                            let encounter = self.encounters.get(&id).unwrap();
                            let cmd =
                                CombatTurnMessage::EncounterState(encounter.lock().await.clone());
                            tx.send(cmd).unwrap();
                        }
                        None => {
                            error!("Could not find encounter for combatant: {:?}", combatant_id);
                            return;
                        }
                    }
                }
                NotifyPlayers {
                    message,
                    recipient_ids,
                } => {
                    // Notify players, similar to your existing notify_players logic
                } // Handle other messages
            }
        }
    }

    pub fn create_npc_encounter(&mut self, hero: Hero, npc: Monster) {
        let encounter = CombatEncounter::new(hero, npc);
        self.add_encounter(encounter);
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
// impl<'de> Deserialize<'de> for CombatCommand {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?; // Deserialize as a String first
//
//         // Match against potential command patterns
//         if s == "EnterBattle" {
//             // Construct & return the EnterBattle variant
//             Ok(CombatCommand::EnterBattle(None))
//             // Replace `PlayerDecisionMaker::default()` with your actual construction logic
//         } else if s.starts_with("Attack(") {
//             Ok(CombatCommand::Attack)
//         } else if s.starts_with("UseTalent(") {
//             // Extract target_id and talent (deserialize)
//             let parts: Vec<&str> = s
//                 .trim_start_matches("UseTalent(")
//                 .trim_end_matches(')')
//                 .split(",")
//                 .collect();
//             if parts.len() != 2 {
//                 return Err(serde::de::Error::custom("Invalid UseTalent command format"));
//             }
//             let target_id = parts[0].to_string();
//             let talent = serde_json::from_str(parts[1]); // Assumes Talent can be deserialized
//             match talent {
//                 Ok(talent) => Ok(CombatCommand::UseTalent(target_id, talent)),
//                 Err(e) => Err(serde::de::Error::custom(format!(
//                     "Failed to deserialize talent: {}",
//                     e
//                 ))),
//             }
//             // Ok(CombatCommand::UseTalent(target_id, talent))
//         } else {
//             Err(serde::de::Error::custom("Unknown command"))
//         }
//     }
// }
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
