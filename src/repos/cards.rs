use prisma_client_rust::QueryError;
use tracing::error;

use crate::{
    models::cards::Card,
    prisma::{
        card::{self, minion_effects, spell_effects},
        damage_effect, hero, hero_card, minion_effect, minion_effect_effect, spell_effect,
        spell_effect_effect,
    },
    webserver::get_prisma_client,
};

pub struct CardRepo {}

impl CardRepo {
    pub async fn get_all_cards() -> Vec<Card> {
        let prisma = get_prisma_client();
        let cards = prisma
            .card()
            .find_many(vec![])
            .with(CardRepo::fetch_minion_effects())
            .with(CardRepo::fetch_spell_effects())
            .exec()
            .await;
        match cards {
            Ok(cards) => cards.into_iter().map(|card| card.into()).collect(),
            Err(e) => {
                error!("Failed to get all cards: {:?}", e);
                vec![]
            }
        }
    }

    pub async fn get_all_hero_cards(hero_id: String) -> Vec<Card> {
        let prisma = get_prisma_client();
        // let _where = vec![deck::hero::is(vec![hero::id::equals(hero_id)])];
        // let hero_deck = prisma.deck().find_first(_where).exec().await;
        // let id = match hero_deck {
        //     Ok(hero_deck) => {
        //         let hero_deck = hero_deck.unwrap();
        //         hero_deck.id
        //     }
        //     Err(e) => {
        //         error!("Failed to get all hero cards: {:?}", e);
        //         return vec![];
        //     }
        // };
        let hero_cards = prisma
            .hero_card()
            .find_many(vec![hero_card::hero_id::equals(hero_id)])
            .with(
                hero_card::card::fetch()
                    .with(CardRepo::fetch_minion_effects())
                    .with(CardRepo::fetch_spell_effects()),
            )
            .exec()
            .await;
        match hero_cards {
            Ok(cards) => cards.into_iter().map(|card| card.into()).collect(),
            Err(e) => {
                error!("Failed to get all hero cards: {:?}", e);
                vec![]
            }
        }
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

    pub fn fetch_minion_effects() -> minion_effects::Fetch {
        card::minion_effects::fetch(vec![]).with(
            minion_effect::effects::fetch(vec![])
                .with(minion_effect_effect::pickup_effect::fetch())
                .with(minion_effect_effect::resilience_effect::fetch())
                .with(minion_effect_effect::poison_effect::fetch())
                .with(minion_effect_effect::taunt_effect::fetch())
                .with(minion_effect_effect::lifesteal_effect::fetch())
                .with(minion_effect_effect::summon_effect::fetch())
                .with(minion_effect_effect::charge_effect::fetch()),
        )
    }

    pub fn fetch_spell_effects() -> spell_effects::Fetch {
        card::spell_effects::fetch(vec![]).with(
            spell_effect::effects::fetch(vec![])
                .with(spell_effect_effect::heal_effect::fetch())
                .with(spell_effect_effect::armor_effect::fetch())
                .with(
                    spell_effect_effect::damage_effect::fetch()
                        .with(damage_effect::damage::fetch(vec![])),
                )
                .with(spell_effect_effect::resilience_effect::fetch())
                .with(spell_effect_effect::poison_effect::fetch())
                .with(spell_effect_effect::initiative_effect::fetch())
                .with(spell_effect_effect::stun_effect::fetch()),
        )
    }
}
