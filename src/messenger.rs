use tokio::sync::mpsc;

use crate::{
    events::{
        handle_channeling::ChannelingHandler, handle_costs::CostHandler,
        handle_explore::ExploreHandler, handle_lootbox::LootBoxHandler, handle_quest::QuestHandler,
    },
    infra::Infra,
    models::resources::Resource,
    run_parallel,
    services::tasks::{
        action_names::{Command, TaskAction},
        explore::ExploreAction,
    },
    utils::e400,
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
                // Just a default value -- not used
                let mut task_action = TaskAction::Explore(ExploreAction::default());
                // End Default
                match cmd {
                    Explore {
                        hero_id,
                        region_name,
                        resp,
                    } => ExploreHandler::hero_explores(hero_id, region_name, resp),
                    ExploreCompleted(action) => {
                        task_action = TaskAction::Explore(action.clone());
                        run_parallel!(
                            (task_action.clone());
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
                        task_action = TaskAction::Channel(action.clone());
                        run_parallel!((action.to_owned());LootBoxHandler::create_lootbox_channel,ChannelingHandler::channel_completed);
                    }
                    QuestAccepted {
                        hero_id,
                        quest_id,
                        resp,
                    } => {
                        let quest = Infra::repo()
                            .get_quest_by_id(quest_id.clone())
                            .await
                            .unwrap();
                        let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                        let has_sufficient_shards = match hero.resources.get(&Resource::StormShard)
                        {
                            Some(shards) => *shards >= quest.cost,
                            None => false,
                        };

                        if !has_sufficient_shards {
                            resp.send(Err(anyhow::Error::msg("not enough shards")))
                                .unwrap();
                            return;
                        }

                        task_action = TaskAction::QuestAccepted(hero_id.clone(), quest_id);
                        resp.send(Ok(())).unwrap();
                    }

                    QuestAction {
                        hero_id,
                        action_id,
                        resp,
                    } => QuestHandler::quest_action(hero_id, action_id, resp),
                    QuestActionDone(hero_id, action_id) => {
                        QuestHandler::quest_action_done(hero_id.clone());
                        task_action = TaskAction::QuestAction(hero_id, action_id);
                    }
                    QuestCompleted(hero_id, quest) => {
                        task_action = TaskAction::QuestComplete(hero_id.clone(), quest.clone());
                        run_parallel!((hero_id.to_owned(), quest.to_owned()); QuestHandler::quest_completed, LootBoxHandler::create_lootbox_quest_complete);
                    }
                    _ => {
                        task_action = TaskAction::Explore(ExploreAction::default());
                        // Replace with an appropriate default
                    }
                }
                CostHandler::deduct_action_costs(task_action);
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
