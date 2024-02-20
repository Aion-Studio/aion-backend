use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;
use tokio::join;
use tracing::info;

use crate::services::tasks::action_names::ResponderType;
use crate::{
    infra::Infra,
    logger::Logger,
    messenger::MESSENGER,
    models::quest::Quest,
    services::tasks::action_names::{ActionNames, Command},
    webserver::AppState,
};

#[post("/quests")]
async fn add_quest(quest: Json<Quest>) -> impl Responder {
    let quest = quest.into_inner();
    match Infra::repo().add_quest(quest).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// #[get("/quests/hero/{hero_id}")]
// async fn get_hero_quests(path: Path<String>) -> impl Responder {
//     let hero_id = path.into_inner();
//     match Infra::repo().get_available_quest(hero_id).await {
//         Ok(quests) => HttpResponse::Ok().json(quests),
//         Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
//     }
// }

#[get("/quests/hero/{hero_id}")]
async fn get_hero_quests(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    match Infra::repo().get_available_quest(hero_id).await {
        Ok((quest, hero_quest)) => {
            let mut quest_map = serde_json::to_value(quest).unwrap_or_default();
            let hero_quest_map = serde_json::to_value(hero_quest).unwrap_or_default();

            if let Some(obj) = quest_map.as_object_mut() {
                for (key, value) in hero_quest_map.as_object().unwrap() {
                    obj.insert(key.clone(), value.clone());
                }
            }

            HttpResponse::Ok().json(quest_map)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/quests/hero/{hero_id}/{quest_id}/accept")]
async fn accept_quest(path: Path<(String, String)>) -> impl Responder {
    let (hero_id, quest_id) = path.into_inner();
    let response = tokio::spawn(async move {
        let (tx, resp_rx) = tokio::sync::oneshot::channel();
        info!(
            "Attempting to accept quest for hero {} and quest_id {}",
            hero_id, quest_id
        );
        MESSENGER.send(Command::QuestAccepted {
            hero_id: hero_id.clone(),
            quest_id: quest_id.clone(),
            resp: tx,
        });
        match resp_rx.await {
            Ok(result) => result, // Unwraps the outer Ok, result is now either Ok(_) or Err(_) from your application logic
            Err(e) => Err(e.into()), // Converts RecvError to your application error type if needed
        }
    });

    match response.await {
        Ok(Ok(_)) => HttpResponse::Ok().json(json!({"message": "OK"})),
        Ok(Err(e)) => {
            // Handle application logic error, e.g., not enough shards
            HttpResponse::BadRequest().json(json!({"error": e.to_string()}))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()), // Handle RecvError or task spawn error
    }
}

#[post("/quests/action/{hero_id}/{action_id}")]
async fn do_quest_action(path: Path<(String, String)>, app: Data<AppState>) -> impl Responder {
    let (hero_id, action_id) = path.into_inner();
    let repo = Infra::repo();
    let action_result = repo.get_action_by_id(&action_id);
    let already_done_result = repo.is_action_completed(hero_id.clone(), action_id.clone());
    let result = join!(action_result, already_done_result);

    let (action, already_done) = result;
    if action.is_err() {
        return HttpResponse::InternalServerError().json(json!({"message":"Bad action id"}));
    }
    let action = action.unwrap();
    // double check quest was paid for
    match repo
        .get_hero_quest(
            action.clone().quest.unwrap().id.unwrap().clone(),
            hero_id.clone(),
        )
        .await
    {
        Ok(quest) => {
            if !quest.accepted {
                return HttpResponse::Forbidden().json(json!({"message":"Quest not accepted"}));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };

    match already_done {
        Ok(true) => {
            return HttpResponse::Forbidden().json(json!({"message":"Action already done"}));
        }

        Ok(false) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }

    let response = tokio::spawn(async move {
        let (tx, resp_rx) = tokio::sync::oneshot::channel();
        MESSENGER.send(Command::QuestAction {
            hero_id: hero_id.clone(),
            action_id: action_id.clone(),
            resp: tx,
            combat_tx: app.combat_tx.clone(),
        });
        Logger::log(json!({
            "name":ActionNames::Quest.to_string(),
            "hero_id":hero_id,
            "action":action
        }));

        let res = resp_rx.await;
        res
    });

    match response.await {
        Ok(Ok(responder_type)) => match responder_type {
            ResponderType::StringResponse(s) => HttpResponse::Ok().json(json!({"token": s})),
            ResponderType::UnitResponse(()) => HttpResponse::Ok().json(json!({"message": "OK"})),
        },
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
