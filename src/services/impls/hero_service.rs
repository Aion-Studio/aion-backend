use crate::models::hero::{Hero, Item, RetinueSlot};
use crate::prisma::PrismaClient;
use crate::repos::hero_repo::HeroRepo;
use crate::services::traits::hero_service::HeroService;
use crate::types::RepoFuture;
use actix_web::web::Data;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceHeroes {
    repo: HeroRepo,
}

impl ServiceHeroes {
    pub fn new(prisma: Data<PrismaClient>) -> Self {
        let repo = HeroRepo::new(Arc::new(prisma));
        Self { repo }
    }
}

impl HeroService for ServiceHeroes {
    fn create_hero<'a>(&'a self, new_hero: Hero) -> RepoFuture<'a, Hero> {
        Box::pin(async move {
            match self.repo.create(new_hero).await {
                Ok(hero) => Ok(hero),
                Err(e) => {
                    eprintln!("Error creating hero: {}", e);
                    Err(e)
                }
            }
        })
    }

    fn level_up_hero<'a>(&'a self, hero: Hero) -> RepoFuture<'a, Hero> {
        Box::pin(async move {
            match self.repo.update_level(hero).await {
                Ok(hero) => Ok(hero),
                Err(e) => {
                    eprintln!("Error updating hero: {}", e);
                    Err(e)
                }
            }
        })
    }

    fn add_experience(&self, hero: &mut Hero, xp: i32) {
        hero.gain_experience(xp);
    }

    fn equip(&self, hero: &mut Hero, item: Item) {
        hero.equip(item);
    }

    fn assign_follower_to_hero(&self, hero: &mut Hero, slot: RetinueSlot) {
        hero.assign_follower(slot);
    }
}
