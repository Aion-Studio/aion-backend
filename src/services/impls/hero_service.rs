use crate::models::hero::{Hero, Item, RetinueSlot};
use crate::prisma::PrismaClient;
use crate::repos::hero_repo::HeroRepo;
use crate::types::RepoFuture;
use std::sync::Arc;
use prisma_client_rust::QueryError;
use crate::models::task::RegionActionResult;

#[derive(Clone, Debug)]
pub struct ServiceHeroes {
    repo: HeroRepo,
}

impl ServiceHeroes {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        let repo = HeroRepo::new(prisma);
        Self { repo }
    }
    pub fn create_hero<'a>(&'a self, new_hero: Hero) -> RepoFuture<'a, Hero> {
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

    pub async fn latest_action_results(&self, hero_id: String) -> Result<Vec<RegionActionResult>, QueryError> {
        self.repo
            .action_results_by_hero(hero_id).await
    }

    pub fn get_hero(&self, hero_id: String) -> RepoFuture<Hero> {
        Box::pin(async move {
            match self.repo.get_hero(hero_id).await {
                Ok(hero) => {
                    let last_action = self.repo.latest_action_result(hero.get_id()).await;
                    match last_action {
                        Ok(action) => {
                            let mut hero = hero.clone();
                            hero.regenerate_stamina(action);
                            let updated = self.repo.update_hero(hero.clone()).await;
                            updated
                        }
                        Err(e) => {
                            eprintln!("Error getting last action: {}", e);
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error getting hero: {}", e);
                    Err(e)
                }
            }
        })
    }

    pub fn level_up_hero<'a>(&'a self, hero: Hero) -> RepoFuture<'a, Hero> {
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
