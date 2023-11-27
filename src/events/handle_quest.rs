use std::sync::Arc;

use futures::TryFutureExt;

use futures::future::{self, FutureExt};
use tokio::spawn;
use tracing::{info, log::error};

use crate::services::tasks::action_names::ActionNames;
use crate::{infra::Infra, models::quest::Quest, services::tasks::explore::ExploreAction};

use super::{dispatcher::EventHandler, game::GameEvent};

#[derive(Clone, Debug)]
pub struct QuestHandler {}

impl QuestHandler {
    pub fn new() -> Self {
        let handler = Self {};
        handler.subscribe();
        handler
    }

    fn subscribe(&self) {
        Infra::subscribe(GameEvent::quest_action(), Arc::new(self.clone()));
        Infra::subscribe(GameEvent::quest_action_done(), Arc::new(self.clone()));
    }

    async fn _handle(&self, event: GameEvent) {
        match event {
            GameEvent::QuestAction(hero_id, action_id) => {
                // unwrap safe here because validation done before action dispatched
                let action = Infra::repo().get_action_by_id(&action_id).await.unwrap();

                let region = action.region_name;

                // marks action as done
                match Infra::repo()
                    .add_hero_action(hero_id.clone(), action_id.clone())
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }

                match ActionNames::from_str(&action.name) {
                    Some(ActionNames::Explore) => {
                        let hero = Infra::repo().get_hero(hero_id.clone()).await.unwrap();
                        let action = ExploreAction::new(hero, region);
                        Infra::dispatch(GameEvent::HeroExplores(action.unwrap()));
                    }
                    Some(ActionNames::Channel) => {}
                    Some(ActionNames::Raid) => {}
                    Some(ActionNames::Unique(off_beat)) => {}
                    _ => {
                        error!("Action name not found");
                        return;
                    }
                };
                Infra::dispatch(GameEvent::QuestActionDone(hero_id));
            }
            GameEvent::QuestActionDone(hero_id) => {
                let repo = Infra::repo();
                let repo_clone = repo.clone();
                let hero_id_clone = hero_id.clone();
                let mut found_quest: Quest = Quest::default();
                let is_done = repo
                    .get_quest_by_hero_id(hero_id.clone())
                    .and_then(|quest| {
                        found_quest = quest.clone();
                        repo.get_quest_action_ids(quest)
                    })
                    .and_then(|action_ids| async move {
                        let hero_completed_actions_ids = repo_clone
                            .get_hero_actions_by_hero_id(hero_id_clone)
                            .await
                            .unwrap();

                        Ok(action_ids.iter().all(|id| {
                            hero_completed_actions_ids
                                .iter()
                                .any(|action| *action == *id)
                        }))
                    })
                    .await;

                match is_done {
                    Ok(done) => {
                        if done {
                            let _ = Infra::repo()
                                .mark_quest_complete(
                                    hero_id.clone(),
                                    found_quest.id.as_ref().unwrap(),
                                )
                                .await;
                            info!("Quest marked as complete. Dispatching complete event...");
                            Infra::dispatch(GameEvent::QuestComplete(hero_id, found_quest));
                        }
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            }
            GameEvent::QuestComplete(hero_id, quest_id) => {
                info!("should enable next quest ? for hero {} ", hero_id);
            }

            _ => {}
        }
    }
}

impl EventHandler for QuestHandler {
    fn handle(&self, event: GameEvent) {
        println!("before box");
        let this = self.clone();
        spawn(async move {
            info!("handling quest event: {:?}", event);
            this._handle(event).await;
        });
    }
}
