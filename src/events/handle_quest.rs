use std::sync::Arc;

use futures::TryFutureExt;
use tracing::{info, log::error};

use crate::{
    configuration::{get_durations, DurationType},
    infra::Infra,
    models::quest::Quest,
    services::tasks::explore::ExploreAction,
};

use super::{
    dispatcher::EventHandler,
    game::{ActionNames, GameEvent},
};

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
    }

    async fn _handle(&self, event: GameEvent) {
        println!("handling quest event: {:?}", event);
        match event {
            GameEvent::QuestAction(hero_id, action_id) => {
                // unwrap safe here because validation done before action dispatched
                let action = Infra::repo().get_action_by_id(&action_id).await.unwrap();

                let region = action.region_name;

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
                        let action = ExploreAction::without_cost(hero, region);
                        Infra::dispatch(GameEvent::HeroExplores(action));
                    }
                    Some(ActionNames::Channel) => {}
                    Some(ActionNames::Raid) => {}
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
                        repo.get_question_action_ids(Some(quest))
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
                                .mark_quest_complete(hero_id.clone(), found_quest.id.unwrap())
                                .await;
                            Infra::dispatch(GameEvent::EnableNextQuest(hero_id));
                        }
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                    }
                }
            }
            GameEvent::EnableNextQuest(hero_id) => {
                info!("enabling next quest for hero {}", hero_id);
            }

            _ => {}
        }
    }
}

impl EventHandler for QuestHandler {
    fn handle(&self, event: GameEvent) {
        let _ = Box::pin(async move {
            info!("handling quest event: {:?}", event);
            self._handle(event).await;
        });
    }
}
