use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum OffBeatActions {
    SlayDragonQuest,
}
