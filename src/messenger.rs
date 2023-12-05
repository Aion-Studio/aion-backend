use tokio::sync::mpsc;

use crate::{
    events::{
        handle_channeling::ChannelingHandler, handle_explore::ExploreHandler,
        handle_lootbox::LootBoxHandler, handle_quest::QuestHandler,
    },
    run_parallel,
    services::tasks::action_names::Command,
};
use once_cell::sync::Lazy;

pub static MESSENGER: Lazy<MessageManager> = Lazy::new(|| MessageManager::new());

pub struct MessageManager {
    tx: mpsc::Sender<Command>,
}

impl MessageManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(10000);
        let _ = Self::init_listener(rx);
        MessageManager { tx }
    }

    fn init_listener(mut rx: mpsc::Receiver<Command>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            use Command::*;

            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Explore {
                        hero_id,
                        region_name,
                        resp,
                    } => ExploreHandler::hero_explores(hero_id, region_name, resp),
                    ExploreCompleted(action) => {
                        run_parallel!(
                            (action.to_owned());
                            ExploreHandler::explore_completed,
                            LootBoxHandler::create_lootbox_explore
                        );
                    }
                    Channel {
                        hero_id,
                        leyline_name,
                        durations,
                        resp,
                    } => ChannelingHandler::hero_channels(hero_id, leyline_name, durations, resp),
                    ChannelCompleted(action) => {
                        run_parallel!((action); LootBoxHandler::create_lootbox_channel);
                    }

                    QuestAction {
                        hero_id,
                        action_id,
                        resp,
                    } => QuestHandler::quest_action(hero_id, action_id, resp),
                    QuestActionDone(hero_id) => QuestHandler::quest_action_done(hero_id),
                    QuestCompleted(hero_id, quest) => {
                        run_parallel!((hero_id.to_owned(), quest.to_owned()); QuestHandler::quest_completed, LootBoxHandler::create_lootbox_quest_complete);
                    }
                }
            }
        })
    }

    pub fn send(&self, message: Command) {
        let tx = self.tx.clone(); // Clone the sender for the async block
        tokio::spawn(async move {
            let _ = tx.send(message).await; // Send the message and ignore the result
        });
    }
}
