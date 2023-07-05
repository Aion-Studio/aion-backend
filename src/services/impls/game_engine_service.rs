use std::{future::Future, pin::Pin, sync::Arc};

use rand::Rng;

use crate::{
    models::{
        hero::{Attributes, BaseStats, Hero, Range},
        region::TaskResult,
    },
    prisma::PrismaClient,
    repos::game_engine::GameEngineRepo,
    services::traits::game_engine::GameEngine,
    types::AsyncResult,
};
use actix_web::web::Data;

use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct GameEngineService {
    tx: Sender<TaskResult>,
    repo: GameEngineRepo,
}

impl GameEngineService {
    pub fn new(prisma: Data<PrismaClient>) -> Arc<Self> {
        let (tx, rx) = mpsc::channel(1200); // create a channel with a buffer size of 1200
        let repo = GameEngineRepo::new(Arc::new(prisma.clone()));
        let service = Arc::new(Self { tx, repo });
        let service_clone = Arc::clone(&service); // Clone the Arc
        let fut = service_clone.listen_for_results(rx); // create the future that listens for completions
        tokio::spawn(fut);
        service
    }
}

impl GameEngine for GameEngineService {
    fn result_channels(&self) -> Result<Sender<TaskResult>, Box<dyn std::error::Error>> {
        Ok(self.tx.clone())
    }

    fn listen_for_results(
        self: Arc<Self>,
        mut rx: Receiver<TaskResult>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            /* Here we add all the cases for every type of action and 
             * update the state of heroes and data models accordingly.
             *
             *
             *
             */
            while let Some(result) = rx.recv().await {
                match result {
                    TaskResult::Region(result) => {
                        if let Err(err) = self
                            .repo
                            .clone()
                            .store_region_action_result(result.clone())
                            .await
                        {
                            eprintln!("Error storing region action result: {}", err);
                            return;
                        }
                        println!("Exploration result: {:?}", result);
                    }
                }
            }
        })
    }

    fn generate_hero(&self) -> AsyncResult<Hero, Box<dyn std::error::Error>> {
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
}
