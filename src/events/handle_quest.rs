use futures::TryFutureExt;

use tokio::sync::oneshot;
use tracing::{info, log::error};

use crate::messenger::MESSENGER;
use crate::services::tasks::action_names::{ActionNames, Command, Responder};
use crate::{infra::Infra, models::quest::Quest};

#[derive(Clone, Debug)]
pub struct QuestHandler {}

impl QuestHandler {
    pub fn quest_action(hero_id: String, action_id: String, resp: Responder<()>) {
        tokio::spawn(async move {
            info!("Quest action started for hero {} and action_id {} ", hero_id,action_id);
            let mut errs = vec![];
            let action = match Infra::repo().get_action_by_id(&action_id).await {
                Ok(action) => action,
                Err(e) => {
                    error!("Error: {}", e);
                    resp.send(Err(e.into())).unwrap();
                    return;
                }
            };

            let region = action.region_name;

            match Infra::repo()
                .add_hero_action(hero_id.clone(), action_id.clone())
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    error!("Error: {}", e);
                    errs.push(e);
                }
            }

            match ActionNames::from_str(&action.name) {
                Some(ActionNames::Explore) => {
                    let (resp_tx, resp_rx): (Responder<()>, _) = oneshot::channel();

                    let cmd = Command::Explore {
                        hero_id: hero_id.clone(),
                        region_name: region,
                        resp: resp_tx,
                    };
                    MESSENGER.send(cmd);
                    let result = resp_rx.await;
                    match result {
                        Ok(_) => info!("Explore action kicked off from quest action..."),
                        Err(e) => error!("Error: {:?}", e),
                    }
                }
                Some(ActionNames::Channel) => {}
                Some(ActionNames::Raid) => {}
                Some(ActionNames::Unique(off_beat)) => {}
                _ => {
                    error!("Action name not found");
                    return;
                }
            };
            resp.send(Ok(())).unwrap();
            let done_cmd = Command::QuestActionDone(hero_id);
            MESSENGER.send(done_cmd);
        });
    }

    pub fn quest_action_done(hero_id: String) {
        tokio::spawn(async move {
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
                            .mark_quest_complete(hero_id.clone(), found_quest.id.as_ref().unwrap())
                            .await;
                        info!("Quest marked as complete. Dispatching complete event...");
                        MESSENGER.send(Command::QuestCompleted(hero_id, found_quest));
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        });
    }

    pub fn quest_completed(hero_id: String, quest: Quest) {
        tokio::spawn(async move {
            info!("should enable next quest ? for hero {} ", hero_id);
        });
    }
}
