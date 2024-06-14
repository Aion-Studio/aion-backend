use futures::future::join_all;
use prisma_client_rust::QueryError;
use tracing::error;
use tracing::info;

use crate::models::cards::Deck;
use crate::prisma::deck;
use crate::prisma::deck_card;
use crate::{
    models::cards::{Card, CardEffect, HeroCard},
    prisma::{card, hero, hero_card},
    webserver::get_prisma_client,
};

pub struct CardRepo {}

impl CardRepo {
    pub async fn get_all_cards() -> Vec<Card> {
        let prisma = get_prisma_client();

        let cards = prisma
            .card()
            .find_many(vec![])
            .exec()
            .await
            .unwrap_or(vec![]);

        let cards_with_effects = cards
            .into_iter()
            .map(|card_data| CardRepo::fetch_card_with_effects(card_data))
            .collect::<Vec<_>>();

        let cards_fetched = futures::future::join_all(cards_with_effects)
            .await
            .into_iter()
            .map(|card| card.unwrap())
            .collect();
        cards_fetched
    }

    pub async fn get_hero_decks(hero_id: String) -> Vec<Deck> {
        let prisma = get_prisma_client();

        let decks = prisma
            .deck()
            .find_many(vec![deck::hero_id::equals(Some(hero_id.clone()))])
            .exec()
            .await
            .unwrap_or(vec![]);
        let hero_id = hero_id.clone();

        let all_decks_futures = decks
            .into_iter()
            .map(|deck| {
                let hero_id_ref = &hero_id;
                async move {
                    let cards: Vec<Card> = CardRepo::deck_cards_by_deck_id(deck.id.clone()).await;
                    Deck {
                        id: deck.id,
                        name: deck.name,
                        hero_id: Some(hero_id_ref.clone()),
                        cards_in_deck: cards,
                        active: deck.active,
                    }
                }
            })
            .collect::<Vec<_>>();
        let decks = join_all(all_decks_futures).await;
        decks
    }

    pub async fn get_all_hero_cards(hero_id: String) -> Vec<HeroCard> {
        let prisma = get_prisma_client();

        let hero_cards = prisma
            .hero_card()
            .find_many(vec![
                hero_card::hero_id::equals(hero_id),
                hero_card::deck_card_id::equals(None),
            ])
            .with(hero_card::card::fetch()) // Only fetch the card data initially
            .exec()
            .await;

        match hero_cards {
            Ok(cards) => {
                let cards_with_effects = cards
                    .into_iter()
                    .map(|card| async move {
                        let card_with_effects =
                            CardRepo::fetch_card_with_effects(*(card.card.unwrap())).await;
                        HeroCard {
                            id: card.id,
                            card: card_with_effects.unwrap(), // Assign the fetched card
                        }
                    }) // Unwrap the card data
                    .collect::<Vec<_>>();

                join_all(cards_with_effects)
                    .await
                    .into_iter()
                    .filter_map(|result| Some(result)) // Extract Ok values, discarding errors
                    .collect::<Vec<HeroCard>>()
            }
            Err(e) => {
                error!("Failed to get all hero cards: {:?}", e);
                vec![]
            }
        }
    }

    pub async fn get_hero_card_by_card_id(card_id: String) -> Result<hero_card::Data, QueryError> {
        let prisma = get_prisma_client();
        let hero_card = prisma
            .hero_card()
            .find_first(vec![hero_card::card_id::equals(card_id)])
            .with(hero_card::card::fetch())
            .exec()
            .await?;

        Ok(hero_card.unwrap())
    }

    pub async fn fetch_card_with_effects(card_data: card::Data) -> anyhow::Result<Card> {
        let card = Card::from((card_data, card_effects));
        Ok(card)
    }

    pub async fn add_card(hero_id: String, card_id: String) -> Result<String, QueryError> {
        let prisma = get_prisma_client();
        let hero_card = prisma
            .hero_card()
            .create(hero::id::equals(hero_id), card::id::equals(card_id), vec![])
            .exec()
            .await;
        match hero_card {
            Ok(hero_card) => Ok(hero_card.id),
            Err(e) => {
                error!("Failed to add card: {:?}", e);
                Err(e)
            }
        }
    }

    // toggles a card to be in a deck or not
    pub async fn toggle_deck_status(
        deck_id: String,
        hero_card_id: String,
        to_deck: bool,
    ) -> Result<(), QueryError> {
        let prisma = get_prisma_client();

        if to_deck {
            prisma
                .deck_card()
                .create(
                    deck_id,
                    vec![deck_card::hero_card::connect(hero_card::id::equals(
                        hero_card_id.clone(),
                    ))],
                )
                .exec()
                .await?;
        } else {
            prisma
                .deck_card()
                .delete_many(vec![
                    deck_card::deck_id::equals(deck_id),
                    deck_card::hero_card::is(vec![hero_card::id::equals(hero_card_id)]),
                ])
                .exec()
                .await?;
        }

        Ok(())
    }

    pub async fn remove_hero_card_by_id(deck_card_id: String) -> Result<String, QueryError> {
        let prisma = get_prisma_client();
        let hero_card = prisma
            .hero_card()
            .delete(hero_card::id::equals(deck_card_id))
            .exec()
            .await;
        match hero_card {
            Ok(hero_card) => Ok(hero_card.id),
            Err(e) => {
                error!("Failed to remove card: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn deck_cards_by_deck_id(deck_id: String) -> Vec<Card> {
        let prisma = get_prisma_client();
        let deck_cards = prisma
            .deck_card()
            .find_many(vec![deck_card::deck_id::equals(deck_id.clone())])
            .with(deck_card::hero_card::fetch().with(hero_card::card::fetch()))
            .exec()
            .await;

        match deck_cards {
            Ok(cards) => {
                let cards_with_effects = cards
                    .into_iter()
                    .map(|deck_card| async move {
                        let card_data = deck_card.hero_card.unwrap().unwrap().card.unwrap();
                        CardRepo::fetch_card_with_effects(*card_data).await
                    })
                    .collect::<Vec<_>>();

                futures::future::join_all(cards_with_effects)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>() // Collect the Results
                    .unwrap_or(vec![]) // Handle potential errors
            }
            Err(e) => {
                error!("Error getting deck  cards: {}", e);
                vec![]
            }
        }
    }

    pub async fn create_deck(hero_id: String, deck_name: String) -> Result<Deck, QueryError> {
        let prisma = get_prisma_client();
        let deck = prisma
            .deck()
            .create(vec![
                deck::name::set(deck_name),
                deck::hero_id::set(Some(hero_id)),
            ])
            .with(deck::hero::fetch())
            .exec()
            .await?;
        Ok(deck.into())
    }
}
