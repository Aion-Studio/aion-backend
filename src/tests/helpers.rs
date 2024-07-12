use std::{
    process::Command,
    sync::{Arc, Once},
};

use crate::{
    events::combat::CombatEncounter,
    models::{
        combatant::Combatant,
        hero::Hero,
        hero_combatant::HeroCombatant,
        npc::{CpuCombatantDecisionMaker, Monster},
        player_decision_maker::PlayerDecisionMaker,
    },
    prisma::PrismaClient,
    services::{
        impls::combat_service::{
            CombatCommand, CombatController, ControllerMessage, EnterBattleData,
        },
        traits::combat_decision_maker::DecisionMaker,
    },
};
use actix_web::web::Data;
use futures::channel::oneshot;
use prisma_client_rust::raw;

use lazy_static::lazy_static;
use tokio::sync::{mpsc, Mutex};

lazy_static! {
    static ref SETUP_DONE: Arc<std::sync::Mutex<bool>> = Arc::new(std::sync::Mutex::new(false));
}

lazy_static! {
    static ref URL: Arc<String> = Arc::new(String::from("postgresql://root:root@0.0.0.0:5432"));
}

async fn setup_once() -> Result<(), Box<dyn std::error::Error>> {
    // Put the parts of the setup that you want to run only once here

    let prisma_client = PrismaClient::_builder()
        .with_url(URL.clone().to_string())
        .build()
        .await
        .unwrap();
    let sql = raw!("DROP DATABASE IF EXISTS testdb");
    prisma_client._execute_raw(sql).exec().await?;

    let sql = raw!("CREATE DATABASE testdb");
    prisma_client._execute_raw(sql).exec().await?;

    drop(prisma_client);

    let _output = Command::new("cargo")
        .args(&["prisma", "migrate", "dev"])
        .env("DATABASE_URL", format!("{}/testdb", URL.to_string())) // make sure to use the new database name
        .output()
        .expect("failed to execute process");

    Ok(())
}

pub async fn setup_test_database() -> Result<Data<PrismaClient>, Box<dyn std::error::Error>> {
    {
        let mut setup_done = SETUP_DONE.lock().unwrap();
        if !*setup_done {
            setup_once().await?;
            *setup_done = true;
        }
    }
    // Re-initialize connected to test DB
    let prisma_client = PrismaClient::_builder()
        .with_url(format!("{}/testdb", URL.to_string())) // make sure to use the new database name
        .build()
        .await?;
    let _ = seed_database(&prisma_client).await?;

    Ok(Data::new(prisma_client))
}

async fn seed_database(client: &PrismaClient) -> Result<bool, Box<dyn std::error::Error>> {
    client
        ._execute_raw(raw!(
            r#"
            INSERT INTO "Region" (name) 
            VALUES 
              ('Buzna'),
              ('Dusane'),
              ('Emerald'),
              ('Forest'),
              ('Lindon'),
              ('Veladria'),
              ('Yezer');
        "#
        ))
        .exec()
        .await?;
    let insert_leylines = raw!(
        r#"
    INSERT INTO "public"."Leyline" ("id", "name", "xp_reward", "RegionName", "aion_rate", "discovery_required", "stamina_rate") VALUES
    ('6204d298-34df-487e-9267-e6aed27cf2b8', 'Dusawater', 9, 'Dusane', 2, 40, 8),
    ('7699b133-1762-4eb6-b4a8-ca6f97daabd0', 'Dusaearth', 8, 'Dusane', 10, 41, 3),
    ('26c0affd-99d0-4a1d-9cb5-2900e1b41aeb', 'Dusaglow', 14, 'Dusane', 4, 22, 8),
    ('62fbfa8c-e1ce-48a9-9d21-2e0e834f000c', 'Dusadream', 12, 'Dusane', 7, 15, 3),
    ('cea699bf-8650-461f-9311-1f8b112a3027', 'Dusawind', 17, 'Dusane', 4, 85, 4),
    ('65980570-2d94-4a5d-a4fb-250791bc4381', 'Dusacloud', 11, 'Dusane', 7, 72, 4),
    ('d5e9a122-e053-44f7-9561-e07006d354f8', 'Dusaspark', 6, 'Dusane', 2, 61, 9),
    ('4f44727f-cd0e-4b83-949e-3d73ac07173a', 'Dusafire', 11, 'Dusane', 6, 45, 5),
    ('d17c2a46-28db-4b68-ab6b-635f46b5bbda', 'Dusalight', 3, 'Dusane', 5, 5, 6),
    ('06a706a3-4f74-407a-99c1-a57a634064b8', 'Dusarock', 10, 'Dusane', 8, 0, 4);
        "#
    );
    client._execute_raw(insert_leylines).exec().await?;
    Ok(true)
}

pub fn random_hero() -> Hero {
    Hero::default()
}

//  example
// ._execute_raw(raw!(
//         "INSERT INTO Post (published, title) VALUES ({}, {})",
//         PrismaValue::Boolean(false),
//         PrismaValue::String("A Title".to_string())
//     ))

struct TestRedis {
    url: String,
}

impl TestRedis {
    fn new() -> Self {
        let url = "redis://localhost:6380".to_string(); // Connect to the test Redis instance
        TestRedis { url }
    }
}

lazy_static! {
    static ref TEST_REDIS: Mutex<Option<TestRedis>> = Mutex::new(None);
    static ref INIT: Once = Once::new();
}

pub async fn init_test_redis() -> String {
    INIT.call_once(|| {}); // Ensure initialization happens only once

    let mut redis_guard = TEST_REDIS.lock().await;
    if redis_guard.is_none() {
        let redis = TestRedis::new();
        let url = redis.url.clone();
        *redis_guard = Some(redis);
        url
    } else {
        redis_guard.as_ref().unwrap().url.clone()
    }
}

pub async fn init_test_combat(
    hero: HeroCombatant,
    monster: Monster,
) -> (
    mpsc::Sender<CombatCommand>,
    mpsc::Sender<ControllerMessage>,
    String,
) {
    let redis_url = init_test_redis().await;

    let (controller_tx, controller_rx) = mpsc::channel(100);
    let mut combat_controller = CombatController::new(controller_tx.clone(), &redis_url);

    // Start the controller
    tokio::spawn(async move {
        combat_controller.run(controller_rx).await;
    });

    let hero_id = hero.get_id();

    // Create the encounter
    let encounter = CombatEncounter::new(hero, monster.clone());
    let encounter_id = encounter.get_id();

    // Set up PlayerDecisionMaker
    let (player_tx, mut player_rx) = mpsc::channel(10);
    let decision_maker =
        PlayerDecisionMaker::new(hero_id.clone(), player_tx, Some("test_action".to_string()));

    let player_decision_maker_arc = Arc::new(Mutex::new(decision_maker));
    let cloned = player_decision_maker_arc.clone();

    // Set up CpuCombatantDecisionMaker

    // Add decision makers to the controller
    controller_tx
        .send(ControllerMessage::AddEncounter { encounter })
        .await
        .unwrap();

    // Enter battle always runs before getting player decision maker tx
    controller_tx
        .send(ControllerMessage::Combat((
            CombatCommand::EnterBattle(EnterBattleData(Some(player_decision_maker_arc))),
            hero_id,
        )))
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mutex_locked = cloned.lock().await;
    let player_tx = mutex_locked.get_from_ws_tx().expect("from_tx not set");

    (player_tx, controller_tx, encounter_id)
}
