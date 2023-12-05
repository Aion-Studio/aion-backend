use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OffBeatActions {
    SlayDragonQuest,
}
