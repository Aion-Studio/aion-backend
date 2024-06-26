use futures::TryFutureExt;
use serde_json::json;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tracing::warn;
use tracing::{info, log::error};

use crate::jsontoken::create_combat_token;
use crate::logger::Logger;
use crate::messenger::MESSENGER;
use crate::services::impls::combat_service::ControllerMessage;
use crate::services::tasks::action_names::{
    ActionNames, CmdResponder, Command, Responder, ResponderType,
};
use crate::{infra::Infra, models::quest::Quest};

#[derive(Clone, Debug)]
pub struct QuestHandler {}

impl QuestHandler {
    pub fn quest_action(
        hero_id: String,
        action_id: String,
        resp: CmdResponder<ResponderType>,
        combat_tx: Sender<ControllerMessage>,
    ) {
        tokio::spawn(async move {
            info!(
                "Quest action started for hero {} and action_id {} ",
                hero_id, action_id
            );
            // let mut errs = vec![];
            let action = match Infra::repo().get_action_by_id(&action_id).await {
                Ok(action) => action,
                Err(e) => {
                    error!("Error: {}", e);
                    // resp.send(Err(e.into())).unwrap();
                    return;
                }
            };

            let region = action.region_name;

            Logger::log(json!({"name":ActionNames::Quest.to_string() ,"hero_id": hero_id}));

            match &action.name {
                ActionNames::Explore => {
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
                ActionNames::Raid => {
                    let hero = Infra::hero_repo().get_hero(hero_id.clone()).await.unwrap();
                    let hero_id = hero.get_id();
                    let npc = Infra::repo()
                        .get_npc_by_action_id(&action_id)
                        .await
                        .unwrap();

                    match combat_tx
                        .send(ControllerMessage::CreateNpcEncounter {
                            hero,
                            npc,
                            action_id: action_id.clone(),
                        })
                        .await
                    {
                        Ok(_) => info!("Fight created in controller"),
                        Err(e) => error!("Error sending create npc encounter to controller: {}", e),
                    }

                    info!("created encounter in controller");

                    let token = create_combat_token(hero_id.as_ref());
                    match token {
                        Ok(token) => {
                            info!("Fight created, heres your token {}", token);
                            resp.send(ResponderType::StringResponse(token)).unwrap();
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                            // resp.send(Err(e.into())).unwrap();
                        }
                    }
                }
                ActionNames::Channel => {}
                ActionNames::Unique(_) => {}
                _ => {
                    error!("Action name not found");
                    return;
                }
            };
            // match Infra::repo()
            //     .add_hero_action(hero_id.clone(), action_id.clone())
            //     .await
            // {
            //     Ok(_) => {}
            //     Err(e) => {
            //         error!("Error: {}", e);
            //         errs.push(e);
            //     }
            // }
            // let done_cmd = Command::QuestActionDone(hero_id, action_id);
            // MESSENGER.send(done_cmd);
        });
    }

    pub fn quest_accepted(hero_id: String, quest_id: String, resp: Responder<()>) {
        tokio::spawn(async move {
            let repo = Infra::repo();
            if let Ok(_) = repo.accept_quest(hero_id.clone(), quest_id).await {
                info!("Quest accepted succesfully for hero {}", hero_id);
                resp.send(Ok(())).unwrap();
            } else {
                warn!("Error accepting quest for hero {}", hero_id);
                resp.send(Err(anyhow::Error::msg("Error accepting quest")))
                    .unwrap();
            }
        });
    }

    pub fn quest_action_done(hero_id: String) {
        tokio::spawn(async move {
            let repo = Infra::repo();
            let repo_clone = repo.clone();
            let hero_id_clone = hero_id.clone();
            let mut found_quest: Quest = Quest::default();
            let is_whole_quest_done = repo
                .get_quest_by_hero_id(hero_id.clone())
                .and_then(|(quest, _)| {
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

            match is_whole_quest_done {
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
            Logger::log(json!({"name":"quest-completed" ,"hero_id": hero_id, 
           "quest-name": quest.title }));
            info!("should enable next quest ? for hero {} ", hero_id);
        });
    }
}
