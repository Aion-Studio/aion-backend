use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;
use tokio::join;

use crate::{
    events::game::GameEvent, handlers::response::ApiResponse, infra::Infra, messenger::MESSENGER,
    models::quest::Quest, services::tasks::action_names::Command,
};

#[post("/quests")]
async fn add_quest(quest: Json<Quest>) -> impl Responder {
    let quest = quest.into_inner();
    println!("quest: {:?}", quest);
    match Infra::repo().add_quest(quest).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/quests/hero/{hero_id}")]
async fn get_hero_quests(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    match Infra::repo().get_available_quest(hero_id).await {
        Ok(quests) => HttpResponse::Ok().json(quests),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/quests/action/{hero_id}/{action_id}")]
async fn do_quest_action(path: Path<(String, String)>) -> impl Responder {
    let (hero_id, action_id) = path.into_inner();
    let repo = Infra::repo();
    let action_result = repo.get_action_by_id(&action_id);
    let already_done_result = repo.is_action_completed(hero_id.clone(), action_id.clone());
    let result = join!(action_result, already_done_result);

    let (action, already_done) = result;
    if action.is_err() {
        return HttpResponse::InternalServerError().json(json!({"message":"Bad action id"}));
    }

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
        });
        let res = resp_rx.await;
        res
    });

    match response.await {
        Ok(Ok(_)) => HttpResponse::Ok().json(json!({"message":"OK"})),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
