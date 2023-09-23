use crate::events::game::{ExploreResult, GameEvent, ActionCompleted};
use crate::infra::Infra;
use crate::models::hero::{Attributes, BaseStats, Hero, Item, Range, RetinueSlot};
use crate::prisma::PrismaClient;
use crate::repos::hero_repo::HeroRepo;
use crate::services::tasks::explore::ExploreAction;
use crate::types::{AsyncResult, RepoFuture};
use prisma_client_rust::QueryError;
use rand::Rng;
use std::sync::Arc;

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

    pub async fn latest_action_completed(
        &self,
        hero_id: String,
    ) -> Result<Vec<ActionCompleted>, QueryError> {
        self.repo.action_results_by_hero(hero_id).await
    }

    pub fn generate_hero(&self) -> AsyncResult<Hero, Box<dyn std::error::Error>> {
        Box::pin(async {
            let mut rng = rand::thread_rng();

            let hero = Hero::new(
                BaseStats {
                    id: None,
                    level: 1,
                    xp: 0,
                    damage: Range {
                        min: rng.gen_range(1..5),
                        max: rng.gen_range(5..10),
                    },
                    hit_points: rng.gen_range(90..110),
                    mana: rng.gen_range(40..60),
                    armor: rng.gen_range(5..15),
                },
                Attributes {
                    id: None,
                    strength: rng.gen_range(1..20),
                    resilience: rng.gen_range(1..20),
                    agility: rng.gen_range(1..20),
                    intelligence: rng.gen_range(1..20),
                    exploration: rng.gen_range(1..20),
                    crafting: rng.gen_range(1..20),
                },
                rng.gen_range(80..120),
                0,
            );

            Ok(hero)
        })
    }

    pub fn get_hero(&self, hero_id: String) -> RepoFuture<Hero> {
        Box::pin(async move {
            match self.repo.get_hero(hero_id).await {
                Ok(hero) => {
                    let last_action = self.repo.latest_action_result(hero.get_id()).await;
                    match last_action {
                        Ok(action_result) => {
                            let mut hero = hero.clone();
                            match action_result {
                                Some(action) => {
                                    hero.regenerate_stamina(action);
                                    self.repo.update_hero(hero.clone()).await
                                }
                                None => Ok(hero),
                            }
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

pub struct HeroService {}

impl HeroService {
    pub async fn explore(&self, action: ExploreAction) {
        Infra::dispatch(GameEvent::HeroExplores(action));
    }
}
