use super::explore::ExploreAction;


#[derive(Clone)]
pub enum TaskKind {
    Exploration(ExploreAction),
    // add other kinds of tasks here
}
