use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prisma::quest::SetParam;

use super::region::Leyline;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub region_name: String,
    pub leyline: Option<Leyline>,
    pub quest: Option<Quest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quest {
    pub id: Option<Uuid>,
    pub title: String,
    pub region_name: String,
    pub required_quests: Vec<Quest>,
    pub required_by: Option<Vec<Uuid>>,
    pub quest_number: i32,
    pub actions: Vec<Action>,
}

impl Quest {
    pub fn new(
        id: Option<Uuid>,
        title: String,
        region_name: String,
        required_quests: Vec<Quest>,
        required_by: Option<Vec<Uuid>>,
        quest_number: i32,
        actions: Vec<Action>,
    ) -> Self {
        Quest {
            id,
            title,
            region_name,
            required_quests,
            required_by,
            quest_number,
            actions,
        }
    }
    pub fn set(quest: &Quest) -> Vec<SetParam> {
        vec![SetParam::Title(quest.title.clone())]
    }
}
