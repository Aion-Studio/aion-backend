use tracing::error;

use crate::{infra::Infra, models::hero::Hero};

pub async fn update_hero_db(hero: Hero) {
    if let Err(e) = Infra::hero_repo().update_hero(hero).await {
        error!("Error updating hero: {}", e);
    }
}
