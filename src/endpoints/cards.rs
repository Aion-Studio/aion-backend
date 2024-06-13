use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse, Responder,
};

use crate::repos::cards::CardRepo;

#[get("/all-cards")]
pub async fn get_cards() -> impl Responder {
    let cards = CardRepo::get_all_cards().await;
    HttpResponse::Ok().json(cards)
}

// grab the request body hero_id and card_id
#[derive(serde::Deserialize)]
pub struct AddCardRequest {
    hero_id: String,
    card_id: String,
}

#[derive(serde::Deserialize)]
pub struct CardRequest {
    card_id: String,
}

#[post("/add-card")]
pub async fn add_card(action: web::Json<AddCardRequest>) -> impl Responder {
    let hero_id = action.hero_id.clone();
    let card_id = action.card_id.clone();
    let card = CardRepo::add_card(hero_id, card_id).await;
    HttpResponse::Ok().json(card)
}

#[post("/remove-card")]
pub async fn remove_card(action: web::Json<CardRequest>) -> impl Responder {
    let card_id = action.card_id.clone();
    let card = CardRepo::remove_hero_card_by_id(card_id).await;
    HttpResponse::Ok().json(card)
}

#[get("/hero-cards/{hero_id}")]
pub async fn get_hero_cards(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    let cards = CardRepo::get_all_hero_cards(hero_id).await;
    HttpResponse::Ok().json(cards)
}

#[get("/decks/{hero_id}")]
pub async fn get_hero_decks(path: Path<String>) -> impl Responder {
    let hero_id = path.into_inner();
    let decks = CardRepo::get_hero_decks(hero_id).await;
    HttpResponse::Ok().json(decks)
}

#[post("hero-cards/add-to-deck/{deck_id}/{hero_card_id}")]
pub async fn add_to_deck(path: Path<(String, String)>) -> impl Responder {
    let (deck_id, hero_card_id) = path.into_inner();
    let card_id = CardRepo::toggle_deck_status(deck_id, hero_card_id, true).await;
    HttpResponse::Ok().json(card_id)
}

#[post("decks/{hero_id}/{deck_name}")]
pub async fn create_deck(path: Path<(String, String)>) -> impl Responder {
    let (hero_id, deck_name) = path.into_inner();
    let deck = CardRepo::create_deck(hero_id, deck_name).await;
    HttpResponse::Ok().json(deck)
}

#[post("hero-cards/remove-from-deck/{deck_id}/{card_id}")]
pub async fn remove_from_deck(path: Path<(String, String)>) -> impl Responder {
    let (deck_id, card_id) = path.into_inner();
    let hero_card = CardRepo::get_hero_card_by_card_id(card_id).await;
    let hero_card_id = hero_card.unwrap().id;
    let card_id = CardRepo::toggle_deck_status(deck_id, hero_card_id, false).await;
    HttpResponse::Ok().json(card_id)
}
