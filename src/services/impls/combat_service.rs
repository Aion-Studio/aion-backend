use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use prisma_client_rust::chrono;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::join;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot, Mutex, Notify};
use tracing::error;
use tracing::log::info;

use crate::events::combat::CombatTurnMessage::PlayerState;
use crate::events::combat::CombatantIndex::{Combatant1, Combatant2};
use crate::events::combat::EncounterState;
use crate::models::cards::{Card, CardType};
use crate::models::hero::Hero;
use crate::models::npc::{CpuCombatantDecisionMaker, Monster};
use crate::{
    events::combat::{CombatEncounter, CombatTurnMessage},
    models::talent::Talent,
    services::{tasks::action_names::Responder, traits::combat_decision_maker::DecisionMaker},
};

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
    SendMsgsToPlayer {
        encounter_state: EncounterState,
        combatant_id: String,
        result: CombatTurnMessage,
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
                       use CombatTurnMessage::*;
                            use ControllerMessage::*;

                        while let Some(command) = command_receiver.recv().await {
                           let sender = result_sender.clone();

                            let mut encounter = encounter_clone.lock().await;
                            match encounter.process_combat_turn(command, &combatant_id) {
                                Ok(result) => {
                                    controller_sender.send(
                                            ControllerMessage::NotifyPlayers {
                                                message: result.clone(),
                                                sender: (combatant_id.to_string(), sender.clone()) }
                                    ).await.unwrap();

                                     if !matches!(result, Winner(_)) {

                                        controller_sender.send(SendMsgsToPlayer{
                                            encounter_state: encounter.request_state(),
                                            combatant_id: combatant_id.to_string(),
                                            result: result.clone()
                                            }).await.unwrap();

                                    } else {
                                        info!("Battle is over, we got a winner");
                                        let encounter_id = encounter.get_id().clone();
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
        encounter_state: &EncounterState, // Assuming EncounterState is the type of `state`
        encounter: &CombatEncounter,
    ) -> CombatTurnMessage {
        let (me_idx, opponent_idx) =
            if encounter.get_combatant_idx(combatant_id).unwrap() == Combatant1 {
                (Combatant1, Combatant2)
            } else {
                (Combatant2, Combatant1)
            };

        let (me, opponent) = if *combatant_id == encounter_state.combatant_1.get_id() {
            (
                encounter_state.combatant_1.clone_box(),
                encounter_state.combatant_2.clone_box(),
            )
        } else {
            (
                encounter_state.combatant_2.clone_box(),
                encounter_state.combatant_1.clone_box(),
            )
        };

        let turn = encounter_state.turn.clone();
        let opponent_hp = opponent.get_hp();
        PlayerState {
            me,
            me_idx: me_idx.clone(),
            opponent_hp,
            opponent,
            turn,
            active_effects: encounter_state.active_effects.clone(),
            my_battle_field: encounter_state.battle_fields.get(&me_idx).unwrap().clone(),
            opponent_battle_field: encounter_state
                .battle_fields
                .get(&opponent_idx)
                .unwrap()
                .clone(),
        }
    }

    pub async fn run(&mut self, mut message_receiver: mpsc::Receiver<ControllerMessage>) {
        use CombatTurnMessage::*;
        while let Some(message) = message_receiver.recv().await {
            let sender = self.message_sender.clone();

            match message {
                ControllerMessage::RemoveEncounter { encounter_id } => {
                    self.encounters.remove(&encounter_id);
                }
                ControllerMessage::Combat((command, from_id, _)) => match command {
                    CombatCommand::EnterBattle(battle_data) => {
                        if let Some(decision_maker) = battle_data.0 {
                            self.add_decision_maker(from_id.clone(), decision_maker);
                            let opponent_id: String;
                            let mut npc_dec_maker_started = true;
                            {
                                let encounter =
                                    self.encounter_by_combatant_id(&from_id).await.unwrap();
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
                                        let npc_decision_maker =
                                            Arc::new(Mutex::new(CpuCombatantDecisionMaker::new(
                                                npc.clone(),
                                                encounter_guard.get_id(),
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
                        }
                    }

                    CombatCommand::UseTalent(opponent_id, talent) => {
                        println!("UseTalent: {} {:?}", opponent_id, talent);
                    }
                    _ => {}
                },

                ControllerMessage::RemoveDecisionMakers { encounter_id, resp } => {
                    // Perform cleanup logic here...
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
                    let encounter_id = encounter.get_id();
                    self.encounters
                        .insert(encounter_id, Arc::new(Mutex::new(encounter)));
                }
                ControllerMessage::CreateNpcEncounter {
                    hero,
                    npc,
                    action_id,
                } => {
                    let mut encounter = CombatEncounter::new(hero.to_combatant(), npc);
                    encounter.set_action_id(action_id);
                    self.add_encounter(encounter);
                }
                ControllerMessage::AddDecisionMaker {
                    participant_id,
                    decision_maker,
                } => {
                    self.decision_makers.insert(participant_id, decision_maker);
                }

                ControllerMessage::RequestState { combatant_id, tx } => {
                    let encounter_id = self.encounter_id_by_combatant(&combatant_id).await;
                    match encounter_id {
                        Some(id) => {
                            let mut encounter = self.encounters.get(&id).unwrap().lock().await;
                            let mut state = encounter.request_state();
                            let combatant_idx = encounter.get_combatant_idx(&combatant_id).unwrap();
                            if state.battle_fields.get(&combatant_idx).is_none() {
                                // initialize the deck for this combatant -- will only run the first time request state is called
                                encounter
                                    .initialize(&combatant_id)
                                    .expect("Failed to initialize deck");
                                state = encounter.request_state();
                            };
                            let action_id = encounter.action_id.clone();
                            let player_state = self
                                .construct_player_state(&combatant_id, &state, &encounter)
                                .await;
                            tx.send((Some(player_state), action_id)).unwrap();
                        }
                        None => {
                            tx.send((None, None)).unwrap();
                        }
                    }
                }
                ControllerMessage::NotifyPlayers {
                    message,
                    sender: (id, sender),
                } => {
                    let opponent = self
                        .encounter_by_combatant_id(&id)
                        .await
                        .unwrap()
                        .lock()
                        .await
                        .get_opponent(&id);
                    let opponent_id = opponent.lock().unwrap().get_id();
                    let opponent_sender = self.result_senders.get(&opponent_id);
                    let (player_message, opponent_message) = match message {
                        EncounterState(state) => {
                            let encounter_mut = self.encounter_by_combatant_id(&id).await.unwrap();
                            let encounter = encounter_mut.lock().await;

                            let player_message =
                                self.construct_player_state(&id, &state, &encounter).await;
                            let opponent_message = self
                                .construct_player_state(&opponent_id, &state, &encounter)
                                .await;
                            (player_message, opponent_message)
                        }
                        _ => (message.clone(), message.clone()),
                    };
                    let (_, _) = join!(
                        opponent_sender.unwrap().send(opponent_message),
                        sender.send(player_message)
                    );
                }
                // Use this to construct as many msgs to send to client based on the result of a combat turn
                ControllerMessage::SendMsgsToPlayer {
                    encounter_state,
                    combatant_id,
                    result,
                } => {
                    use CombatTurnMessage::*;

                    let sender = self.result_senders.get(&combatant_id).unwrap();
                    let controller_sender = self.message_sender.clone();

                    if let CombatTurnMessage::CommandPlayed(CombatCommand::PlayCard(card)) = &result
                    {
                        if card.card_type == CardType::Spell {
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }

                    controller_sender
                        .clone()
                        .send(ControllerMessage::NotifyPlayers {
                            message: EncounterState(encounter_state),
                            sender: (combatant_id.to_string(), sender.clone()),
                        })
                        .await
                        .unwrap();
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatCommand {
    EnterBattle(EnterBattleData),
    AttackMinion {
        attacker: Card,
        #[serde(rename = "defenderId")]
        defender_id: String,
    },
    AttackHero(Card),
    AttackExchange {
        attacker_id: String,
        attacker_damage_taken: i32,
        defender_damage_taken: i32,
        defender_id: String,
    },
    UseTalent(String, Talent), // Use a talent: (Talent)
    PlayCard(Card),
    EndTurn,
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
