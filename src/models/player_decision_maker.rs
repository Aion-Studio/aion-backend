use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot, Notify};
use tracing::info;

use crate::events::combat::{CombatTurnMessage, CombatantIndex};
use crate::messenger::MESSENGER;
use crate::services::impls::combat_service::CombatCommand;
use crate::services::tasks::action_names::Command;
use crate::services::traits::combat_decision_maker::DecisionMaker;

#[derive(Debug)]
pub struct PlayerDecisionMaker {
    id: String,
    combat_controller_tx: Option<Sender<CombatCommand>>,
    // provided by combat controller
    pub player_idx: CombatantIndex,
    to_ws_tx: Sender<CombatTurnMessage>,
    from_ws_tx: Option<Sender<CombatCommand>>,
    pub notify_from_ws_tx_set: Arc<Notify>,
    shutdown_signal: Option<oneshot::Receiver<()>>,
    shutdown_trigger: Option<oneshot::Sender<()>>,
    command_receiver_shutdown: Arc<AtomicBool>,
    action_id: Option<String>,
}

impl PlayerDecisionMaker {
    pub fn new(id: String, to_ws_tx: Sender<CombatTurnMessage>, action_id: Option<String>) -> Self {
        let (shutdown_trigger, shutdown_signal) = oneshot::channel();

        let instance = Self {
            combat_controller_tx: None,
            id,
            to_ws_tx,
            player_idx: CombatantIndex::Combatant1,
            from_ws_tx: None,
            notify_from_ws_tx_set: Arc::new(Notify::new()),
            shutdown_signal: Some(shutdown_signal),
            shutdown_trigger: Some(shutdown_trigger),
            command_receiver_shutdown: Arc::new(AtomicBool::new(false)),
            action_id,
        };

        instance
    }

    // Receive command from player websocket, send to combat controller
    pub fn start_listening_for_commands(&mut self) {
        let (command_sender, mut command_receiver) = mpsc::channel(10);
        //set the sender so combat socket can get it via get_from_ws_tx it to send commands from player
        self.from_ws_tx = Some(command_sender);
        let combat_controller_tx = self.combat_controller_tx.clone().unwrap();
        let shutdown_signal = self.command_receiver_shutdown.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    cmd = command_receiver.recv() => {
                        if let Some(cmd) = cmd {
                            info!("Received command from ws_player: {:?}", cmd);
                            if let Err(e) = combat_controller_tx.send(cmd).await {
                                info!("Receiver for player decision maker has shut down ");
                                shutdown_signal.store(true, Ordering::SeqCst);
                                break;
                            }
                        } else {
                            break;
                        }
                    },
                    _ = tokio::signal::ctrl_c() => {
                        // Handle Ctrl+C or another termination signal if needed
                        break;
                    },
                }

                // Check if shutdown signal is set
                if shutdown_signal.load(Ordering::SeqCst) {
                    info!("Shutdown signal received, terminating command receiver loop.");
                    break;
                }
            }
        });
    }
    pub fn get_from_ws_tx(&self) -> Option<Sender<CombatCommand>> {
        self.from_ws_tx.clone()
    }
}

impl DecisionMaker for PlayerDecisionMaker {
    fn start(
        &mut self,
        combat_controller_tx: Sender<CombatCommand>,
        player_idx: CombatantIndex,
    ) -> Sender<CombatTurnMessage> {
        self.player_idx = player_idx;
        self.combat_controller_tx = Some(combat_controller_tx);

        let shutdown_signal = self
            .shutdown_signal
            .take()
            .expect("Shutdown signal must be present when starting.");

        self.start_listening_for_commands();
        self.notify_from_ws_tx_set.notify_one();

        let (command_sender, mut result_receiver): (
            Sender<CombatTurnMessage>,
            Receiver<CombatTurnMessage>,
        ) = mpsc::channel(10);

        let to_ws_tx = self.to_ws_tx.clone();

        let player_idx = self.player_idx.clone();
        let action_id = self.action_id.clone();
        let id = self.id.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = shutdown_signal => {
                    info!("Shutting down decision maker for player.");
                },
                _ = async {
                    while let Some(result) = result_receiver.recv().await {
                    let id = id.clone();

                        let to_ws_tx = to_ws_tx.clone();

                        to_ws_tx.send(result.clone()).await.unwrap();
                          if let CombatTurnMessage::Winner(idx) = &result {

                            if *idx == player_idx && action_id.is_some(){
                               MESSENGER.send(Command::QuestActionDone(id,action_id.clone().unwrap()));
                                info!("Player has won the game.");
                            }
                        }
                    }
                } => {}
            }
        });
        command_sender
    }
    fn get_id(&self) -> String {
        self.id.clone()
    }
    fn shutdown(&mut self) {
        if let Some(trigger) = self.shutdown_trigger.take() {
            let _ = trigger.send(());
        }
    }
}

impl Drop for PlayerDecisionMaker {
    fn drop(&mut self) {
        // Cleanup code here
    }
}
