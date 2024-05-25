use once_cell::sync::Lazy;
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::{
    events::{
        handle_channeling::ChannelingHandler, handle_costs::CostHandler,
        handle_explore::ExploreHandler, handle_lootbox::LootBoxHandler, handle_quest::QuestHandler,
    },
    infra::Infra,
    models::resources::Resource,
    run_parallel,
    services::tasks::action_names::{Command, TaskAction},
};

pub static MESSENGER: Lazy<MessageManager> = Lazy::new(|| MessageManager::new());

pub struct MessageManager {
    tx: mpsc::Sender<Command>,
}

impl MessageManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100000);
        let _ = Self::init_listener(rx);
        MessageManager { tx }
    }

    fn init_listener(mut rx: mpsc::Receiver<Command>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            use Command::*;

            while let Some(cmd) = rx.recv().await {
                // Note: never return from any of these match arms otherwise the listener will stop
                let task_action = match cmd {
                    Explore {
                        hero_id,
                        region_name,
                        resp,
                    } => {
                        ExploreHandler::hero_explores(hero_id, region_name, resp);
                        None
                    }
                    ExploreCompleted(action) => {
                        let task_action = TaskAction::Explore(action.clone());

                        LootBoxHandler::create_lootbox_explore(task_action.clone()).await;
                        Some(task_action)
                    }
                    Channel {
                        hero_id,
                        leyline_name,
                        durations,
                        resp,
                    } => {
                        ChannelingHandler::hero_channels(hero_id, leyline_name, durations, resp);
                        None
                    }
                    ChannelCompleted(action) => {
                        run_parallel!((action.to_owned());LootBoxHandler::create_lootbox_channel,ChannelingHandler::channel_completed);
                        Some(TaskAction::Channel(action.clone()))
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
                            if let Err(e) = resp.send(Err(anyhow::Error::msg("not enough shards")))
                            {
                                warn!("Failed to send response: {:?}", e);
                            }
                            None
                        } else {
                            QuestHandler::quest_accepted(hero_id.clone(), quest_id.clone(), resp);
                            Some(TaskAction::QuestAccepted(hero_id.clone(), quest_id))
                        }
                    }

                    QuestAction {
                        hero_id,
                        action_id,
                        resp,
                        combat_tx,
                    } => {
                        QuestHandler::quest_action(hero_id, action_id, resp, combat_tx);
                        None
                    }
                    QuestActionDone(hero_id, action_id) => {
                        QuestHandler::quest_action_done(hero_id.clone());
                        let task_action = TaskAction::QuestAction(hero_id, action_id);
                        LootBoxHandler::create_lootbox_quest_action(task_action.clone()).await;
                        Some(task_action)
                    }
                    QuestCompleted(hero_id, quest) => {
                        run_parallel!((hero_id.to_owned(), quest.to_owned()); QuestHandler::quest_completed, LootBoxHandler::create_lootbox_quest_complete);
                        Some(TaskAction::QuestComplete(hero_id.clone(), quest.clone()))
                    }
                };
                if let Some(task_action) = task_action {
                    CostHandler::deduct_action_costs(task_action)
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
