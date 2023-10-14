use super::{handle_explore::ExploreHandler, handle_lootbox::LootBoxHandler, handle_channeling::ChannelingHandler};

pub fn initialize_handlers() {

    ExploreHandler::new();
    LootBoxHandler::new();
    ChannelingHandler::new();
}
