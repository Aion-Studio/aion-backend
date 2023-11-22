use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prisma::{
    action,
    quest::{self, actions, SetParam},
};

use super::region::{Leyline, Region, RegionName};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub region_name: RegionName,
    pub leyline: Option<Leyline>,
    pub quest: Option<Quest>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub id: Option<String>,
    pub title: String,
    pub region_name: String,
    pub quest_number: i32,
    pub actions: Vec<Action>,
}

impl Quest {
    pub fn new(
        id: Option<String>,
        title: String,
        region_name: String,
        quest_number: i32,
        actions: Vec<Action>,
    ) -> Self {
        Quest {
            id,
            title,
            region_name,
            quest_number,
            actions,
        }
    }

    pub fn set(quest: &Quest, action_ids: Vec<String>) -> Vec<SetParam> {
        let action_ids: Vec<String> = action_ids
            .into_iter()
            .map(|id| id.to_string()) // Convert Uuid to String
            .collect(); // Collect into a Vec<String>

        let params = vec![actions::connect(
            action_ids
                .iter()
                .map(|id| action::id::equals(id.to_string()))
                .collect(),
        )];

        params
    }
}

impl From<quest::Data> for Quest {
    fn from(data: quest::Data) -> Self {
        Quest {
            id: Some(data.id),
            title: data.title,
            region_name: data.region_name,
            quest_number: data.quest_number,
            actions: match data.actions {
                Some(actions_data) => actions_data.into_iter().map(Action::from).collect(),
                None => Vec::new(),
            },
        }
    }
}

impl From<action::Data> for Action {
    fn from(data: action::Data) -> Self {
        Action {
            id: Some(data.id),
            name: data.name,
            description: data.description,
            region_name: match data.region_name.as_str() {
                "Dusane" => RegionName::Dusane,
                "Yezer" => RegionName::Yezer,
                "Emerlad" => RegionName::Emerlad,
                "Forest" => RegionName::Forest,
                "Buzna" => RegionName::Buzna,
                "Veladria" => RegionName::Veladria,
                "Lindon" => RegionName::Lindon,
                _ => RegionName::Dusane,
            },

            // Handle the double Option and Box for leyline
            leyline: data.leyline.flatten().map(|l| *l).map(Leyline::from),
            // Handle the double Option and Box for quest
            quest: data.quest.flatten().map(|q| *q).map(Quest::from),
            // ... handle other fields as necessary ...
        }
    }
}

impl From<Box<quest::Data>> for Quest {
    fn from(data: Box<quest::Data>) -> Self {
        // Dereference the box to obtain quest::Data and then convert it to Quest
        (*data).into()
    }
}
