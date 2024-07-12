use serde::{Deserialize, Serialize};

use crate::prisma::EffectType;

#[derive(Debug, Clone, Serialize, Deserialize)]
// rename only when serializing to JSON
#[serde(rename_all = "camelCase")]
pub struct ActiveEffect {
    pub effect: EffectType,
    pub remaining_turns: Option<i32>,
}
