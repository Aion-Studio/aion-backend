use tokio::sync::mpsc::{Receiver, Sender};

use crate::{events::combat::CombatTurnResult, services::impls::combat_service::CombatCommand};

pub trait DecisionMaker {
    fn listen_and_make_move(&mut self);
}
