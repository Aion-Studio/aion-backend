use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use crate::{
    models::hero::{Attributes, BaseStats, Hero, Range},
    prisma::PrismaClient,
};
use actix_web::web::Data;
use prisma_client_rust::raw;
use rand::Rng;

use lazy_static::lazy_static;

lazy_static! {
    static ref SETUP_DONE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
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
    let mut rng = rand::thread_rng();

    Hero::new(
        BaseStats {
            id: None,
            level: 1,
            xp: 0,
            damage: Range {
                min: rng.gen_range(1..5),
                max: rng.gen_range(5..10),
            },
            hit_points: rng.gen_range(90..110),
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
    )
}

//  example
// ._execute_raw(raw!(
//         "INSERT INTO Post (published, title) VALUES ({}, {})",
//         PrismaValue::Boolean(false),
//         PrismaValue::String("A Title".to_string())
//     ))
