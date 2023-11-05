use actix_web::{post, web::Json, Responder, HttpResponse};

use crate::{models::quest::Quest, infra::Infra};

#[post("/quests")]
async fn add_quest(quest: Json<Quest>) -> impl Responder {
    let quest = quest.into_inner();
    Infra::repo().add_quest(quest).await.unwrap();
    HttpResponse::Ok().body("OK")
}
