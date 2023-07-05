use std::{sync::Arc, pin::Pin, future::Future};

use rand::Rng;

use crate::{
    models::{
        hero::{Attributes, BaseStats, Hero, Range},
        region::TaskResult,
    },
    services::traits::game_engine::GameEngine,
    types::AsyncResult,
};

use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct GameEngineService {
    tx: Sender<TaskResult>,
}

impl GameEngineService {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = mpsc::channel(1200); // create a channel with a buffer size of 1200

        let service = Arc::new(Self { tx});
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
            while let Some(result) = rx.recv().await {
                match result {
                    TaskResult::Region(result) => {
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
