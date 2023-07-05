use serde::Serialize;

use crate::services::tasks::explore::ExploreAction;


#[derive(Debug,Clone,Serialize)]
pub enum TaskKind {
    Exploration(ExploreAction),
    // add other kinds of tasks here
}

