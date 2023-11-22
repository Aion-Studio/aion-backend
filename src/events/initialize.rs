use super::{handle_explore::ExploreHandler, handle_lootbox::LootBoxHandler, handle_channeling::ChannelingHandler, handle_quest::QuestHandler};

pub fn initialize_handlers() {
    ExploreHandler::new();
    LootBoxHandler::new();
    ChannelingHandler::new();
    QuestHandler::new();
}
