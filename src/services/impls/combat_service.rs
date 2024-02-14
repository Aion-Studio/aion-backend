use std::collections::HashMap;

use anyhow::Error;
use serde::{Serialize, Deserialize};
use tokio::sync::{mpsc, oneshot};
use tracing::error;
use uuid::Uuid;

use crate::{
    events::combat::{CombatEncounter, CombatTurnResult},
    models::talent::Talent,
    services::tasks::action_names::Responder,
};

pub struct CombatController {
    encounters: HashMap<String, CombatEncounter>, // Track in-progress encounters
    command_receiver: mpsc::Receiver<(CombatCommand, String, Responder<CombatTurnResult>)>, // Command , attacker_id
}

impl CombatController {
    pub fn new(
        command_receiver: mpsc::Receiver<(CombatCommand, String, Responder<CombatTurnResult>)>,
    ) -> Self {
        CombatController {
            encounters: HashMap::new(),
            command_receiver,
        }
    }

    // This function now returns the encounter's ID instead of a reference
    fn encounter_id_by_combatant(&self, combatant_id: &str) -> Option<String> {
        self.encounters.iter().find_map(|(id, encounter)| {
            if encounter.has_combatant(combatant_id) {
                Some(id.clone())
            } else {
                None
            }
        })
    }

    pub async fn run(&mut self) {
        while let Some((command, combatant_id, resp)) = self.command_receiver.recv().await {
            let encounter_id = self
                .encounter_id_by_combatant(&combatant_id)
                .expect("Combatant not found in any encounter");

            // Access and mutate the encounter directly
            if let Some(encounter) = self.encounters.get_mut(&encounter_id) {
                // Now you can call a mutable method on encounter
                match encounter.process_combat_turn(command, &combatant_id) {
                    Ok(result) => {
                        resp.send(Ok(result)).unwrap();
                    }
                    Err(e) => {
                        error!("Error: {:?}", e);
                        resp.send(Err(Error::msg(e.to_string()))).unwrap();
                    }
                }
            } else {
                error!("Encounter not found for ID: {}", encounter_id);
            }
        }
    }

    pub fn add_encounter(&mut self, encounter: CombatEncounter) {
        self.encounters.insert(encounter.get_id(), encounter);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatCommand {
    Attack(String),            // Let's say we attack: (CombatantId)
    UseTalent(String, Talent), // Use a talent: (Talent)
}
