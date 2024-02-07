use tracing::error;

use crate::{models::hero::Hero, infra::Infra};


pub async fn update_hero_db(hero: Hero) {
    if let Err(e) = Infra::repo().update_hero(hero).await {
        error!("Error updating hero: {}", e);
    }
}
