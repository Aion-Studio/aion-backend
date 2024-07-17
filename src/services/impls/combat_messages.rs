use std::sync::Arc;

use tokio::join;
use tokio::sync::{mpsc, oneshot, Mutex, Notify};
use tracing::{error, info};

use crate::{
    events::combat::{CombatEncounter, CombatTurnMessage, EncounterState},
    models::npc::{CpuCombatantDecisionMaker, Monster},
};

use super::{
    combat_controller::{CombatCommand, CombatController, ControllerMessage},
    combat_shared_state::SharedState,
};
use ControllerMessage::*;

pub struct MessageHandler {
    state: Arc<Mutex<SharedState>>,
    receiver: mpsc::Receiver<ControllerMessage>,
    controller: Arc<CombatController>,
    sender: mpsc::Sender<ControllerMessage>,
}

impl MessageHandler {
    pub fn new(
        state: Arc<Mutex<SharedState>>,
        controller: Arc<CombatController>,
        receiver: mpsc::Receiver<ControllerMessage>,
        sender: mpsc::Sender<ControllerMessage>,
    ) -> Self {
        Self {
            state,
            receiver,
            controller,
            sender,
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            match self.process_message(message).await {
                Ok(()) => {}
                Err(e) => {
                    error!("Error processing message: {:?}", e);
                }
            }
        }
    }

    async fn process_message(&self, message: ControllerMessage) -> anyhow::Result<()> {
        let sender = self.sender.clone();
        match message {
            ControllerMessage::RemoveEncounter { encounter_id } => {
                if let Err(e) = self.controller.remove_encounter(&encounter_id).await {
                    error!("Error removing encounter: {:?}", e);
                }
            }
            ControllerMessage::StartEncounterForCombatant { combatant_id } => {
                match self.start_encounter_for_combatant(&combatant_id).await {
                    Ok(_) => {
                        println!("Encounter started for combatant: {}", combatant_id)
                    }
                    Err(e) => {
                        error!("Failed to start encounter for combatant: {:?}", e);
                    }
                }
            }
            ControllerMessage::GetCombatant {
                encounter_id,
                combatant_id,
                tx,
            } => {
                let combatant = self
                    .controller
                    .get_combatant_by_encounter(&encounter_id, &combatant_id)
                    .await;
                tx.send(combatant).unwrap();
            }

            ControllerMessage::Combat((command, from_id)) => {
                match command {
                    CombatCommand::EnterBattle(battle_data) => {
                        if let Some(decision_maker) = battle_data.0 {
                            let mut state = self.state.lock().await;
                            state.add_decision_maker(from_id.clone(), decision_maker);
                            drop(state);
                            let opponent_id: String;
                            let mut npc_dec_maker_started = true;
                            {
                                let mut encounter = self
                                    .controller
                                    .encounter_by_combatant_id(&from_id)
                                    .await
                                    .unwrap();
                                let opponent_type = encounter.get_opponent(&from_id);
                                let opponent = opponent_type.as_combatant();
                                opponent_id = opponent.get_id().to_string();
                                if let Some(npc) = opponent.as_any().downcast_ref::<Monster>() {
                                    let state = self.state.lock().await;
                                    let needs_decision_maker =
                                        !state.has_decision_maker(&opponent_id).await;

                                    if needs_decision_maker {
                                        let npc_decision_maker =
                                            Arc::new(Mutex::new(CpuCombatantDecisionMaker::new(
                                                npc.clone(),
                                                encounter.get_id().to_string(),
                                            )));
                                        drop(state);
                                        let mut state = self.state.lock().await;
                                        state.add_decision_maker(
                                            opponent_id.clone(),
                                            npc_decision_maker,
                                        );
                                        npc_dec_maker_started = false;
                                    }
                                }
                            }

                            if !npc_dec_maker_started {
                                self.start_encounter_for_combatant(&opponent_id)
                                    .await
                                    .unwrap();
                            }
                            // let (tx, rx) = oneshot::channel();
                            self.start_encounter_for_combatant(&from_id.clone())
                                .await
                                .unwrap();
                            // rx.await.expect("Failed to start encounter for player");

                            let encounter = self
                                .controller
                                .encounter_by_combatant_id(&from_id)
                                .await
                                .unwrap();
                            self.controller
                                .initialize_encounter(&encounter.get_id())
                                .await
                                .expect("Failed to initialize encounter");
                        }
                    }
                    CombatCommand::LeaveBattle => {
                        if let Some(encounter) =
                            self.controller.encounter_by_combatant_id(&from_id).await
                        {
                            for combatant_id in encounter.get_combatant_ids() {
                                self.sender
                                    .send(ControllerMessage::RemoveSingleDecisionMaker {
                                        combatant_id: combatant_id.to_string(),
                                    })
                                    .await
                                    .unwrap();
                            }
                            info!(
                                "removed decision makers for encounter: {:?}",
                                encounter.get_id()
                            );
                            self.sender
                                .send(ControllerMessage::RemoveEncounter {
                                    encounter_id: encounter.get_id().to_string(),
                                })
                                .await
                                .unwrap();
                        }
                    }
                    // Handle other combat commands...
                    _ => {
                        // Process other combat commands
                    }
                }
            }

            ControllerMessage::RemoveDecisionMakers { encounter_id, resp } => {
                // Perform cleanup logic here...
                let state = self.state.lock().await;
                if let Some(encounter) = state.get_encounter(&encounter_id).await {
                    for combatant_id in encounter.get().get_combatant_ids() {
                        self.sender
                            .send(ControllerMessage::RemoveSingleDecisionMaker { combatant_id })
                            .await
                            .unwrap();
                    }
                }
                resp.send(()).unwrap();
            }
            ControllerMessage::RemoveSingleDecisionMaker { combatant_id } => {
                let mut state = self.state.lock().await;
                if let Some(shutdown_signal) = state.remove_shutdown_signal(&combatant_id) {
                    shutdown_signal.notify_waiters();
                    if let Some(decision_maker) = state.decision_makers.get(&combatant_id) {
                        let mut decision_maker_guard = decision_maker.lock().await;
                        decision_maker_guard.shutdown();
                    }
                    state.remove_decision_maker(&combatant_id);
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
                            Ok(_) => {}
                            Err(e) => {
                                tracing::error!(
                                    "Failed to send RemoveDecisionMakers message: {:?}",
                                    e
                                )
                            }
                        }
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
                if let Err(e) = self.controller.set_encounter(&encounter).await {
                    error!("Failed to add encounter: {:?}", e);
                }
            }

            ControllerMessage::EncounterCheck { combatant_id, tx } => {
                let encounter = self
                    .controller
                    .encounter_by_combatant_id(&combatant_id)
                    .await;
                tx.send(encounter.is_some()).unwrap();
            }
            ControllerMessage::CreateNpcEncounter {
                hero,
                npc,
                action_id,
            } => {
                let mut encounter = CombatEncounter::new(hero.to_combatant(), npc);
                encounter.set_action_id(action_id);
                if let Err(e) = self.controller.set_encounter(&encounter).await {
                    error!("Failed to create NPC encounter: {:?}", e);
                }
            }
            ControllerMessage::AddDecisionMaker {
                combatant_id,
                decision_maker,
            } => {
                let mut state = self.state.lock().await;
                state.add_decision_maker(combatant_id, decision_maker);
            }

            ControllerMessage::RequestState { combatant_id, tx } => {
                if let Some(mut encounter) = self
                    .controller
                    .encounter_by_combatant_id(&combatant_id)
                    .await
                {
                    info!("found encounter for combatant: {}", combatant_id);
                    self.controller
                        .initialize_encounter(&encounter.get_id())
                        .await
                        .expect("Failed to initialize encounter");

                    // Persist the initialized encounter
                    if let Err(e) = self.controller.set_encounter(&encounter).await {
                        error!("Failed to persist initialized encounter: {:?}", e);
                    }

                    let action_id = encounter.action_id.clone();
                    let player_state = self
                        .controller
                        .construct_player_state(&combatant_id, &mut encounter)
                        .await;
                    let npc_state = self
                        .controller
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
                let mut encounter = self
                    .controller
                    .encounter_by_combatant_id(&id)
                    .await
                    .unwrap();

                let opponent_id = encounter.get_opponent(&id).as_combatant().get_id();
                let opponent_sender = self
                    .controller
                    .get_result_sender(&opponent_id)
                    .await
                    .expect("opponent sender not found");

                let var_name = join!(opponent_sender.send(message.clone()), sender.send(message));
                let (_, _) = var_name;
            }
            // Use this to construct as many msgs to send to client based on the result of a combat turn
            ControllerMessage::SendMsgsToPlayer {
                combatant_id,
                result,
            } => {
                let sender = self
                    .controller
                    .get_result_sender(&combatant_id)
                    .await
                    .unwrap();
                let controller_sender = self.sender.clone();

                controller_sender
                    .clone()
                    .send(ControllerMessage::NotifyPlayers {
                        message: result,
                        sender: (combatant_id.to_string(), sender.clone()),
                    })
                    .await
                    .unwrap();

                let encounter = self
                    .controller
                    .encounter_state(combatant_id.clone())
                    .await
                    .unwrap();

                controller_sender
                    .send(ControllerMessage::NotifyPlayers {
                        message: CombatTurnMessage::EncounterData(encounter),
                        sender: (combatant_id.to_string(), sender.clone()),
                    })
                    .await
                    .unwrap();
            }
            ControllerMessage::GetEncounter { combatant_id, tx } => {
                let encounter = self.controller.get_encounter(&combatant_id).await;
                tx.send(encounter).unwrap();
            }
        }
        Ok(())
    }

    pub async fn get_encounter(&self, combatant_id: &str) -> Option<CombatEncounter> {
        let state = self.state.lock().await;
        let encounter = state.get_encounter_by_combatant_id(&combatant_id).await;
        //retun the value of the enc.get() but enc.get() is a &CombatEncounter
        encounter.map(|enc| enc.get().clone())
    }

    pub async fn start_encounter_for_combatant(
        &self,
        combatant_id: &str,
    ) -> Result<(), anyhow::Error> {
        let shutdown_signal = Arc::new(Notify::new());
        {
            let mut state = self.state.lock().await;
            state.add_shutdown_signal(combatant_id.to_string(), shutdown_signal.clone());
        }
        let decision_maker;
        {
            let state = self.state.lock().await;
            decision_maker = state
                .decision_makers
                .get(&combatant_id.to_string())
                .expect("No decision maker found")
                .clone();
        }

        let (command_sender, mut command_receiver) = mpsc::channel(10);
        let encounter = self.get_encounter(combatant_id).await.unwrap();

        let idx = encounter.get_combatant_idx(combatant_id.clone()).unwrap();

        let mut decision_maker_guard = decision_maker.lock().await;

        /***** STARTING NPC DECISION MAKER *****/
        let result_sender = decision_maker_guard.start(command_sender.clone(), idx);
        /***** END STARTING NPC DECISION MAKER *****/

        drop(decision_maker_guard);

        {
            let mut state = self.state.lock().await;
            state.add_result_sender(combatant_id.to_string(), result_sender.clone());
        }
        //
        // self.controller
        //     .add_result_sender(combatant_id.to_string(), result_sender.clone())
        //     .await;

        let controller = self.controller.clone();
        let combatant_id = combatant_id.to_string();

        tokio::spawn(async move {
            tokio::select! {
                _ = shutdown_signal.notified() => {
                    info!("Shutting down listener for combatant: {}", combatant_id);
                },
                _ = async {
                    while let Some(command) = command_receiver.recv().await {
                        let result = controller.process_combat_turn(&combatant_id, command).await;
                        match result {
                            Ok(result) => {
                                if !matches!(result, CombatTurnMessage::Winner(_)) {
                                    if let Err(e) = result_sender.send(result.clone()).await {
                                        error!("Failed to send result to player: {:?}", e);
                                    }
                                } else {
                                    info!("Battle is over, we got a winner");
                                    if let Some(encounter) = controller.encounter_by_combatant_id(&combatant_id).await {
                                        let encounter_id = encounter.get_id().to_string();
                                        if let Err(e) = controller.remove_encounter(&encounter_id).await {
                                            error!("Failed to remove encounter: {:?}", e);
                                        }
                                    }
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
        Ok(())
    }
}
