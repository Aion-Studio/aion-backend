use std::{
    println,
    process::Command,
    sync::{Arc, Once},
};

use crate::{
    models::{
        hero::{Attributes, BaseStats, Hero, Range},
        region::RegionName,
    },
    prisma::PrismaClient,
    services::{tasks::explore::ExploreAction, traits::scheduler::TaskScheduler},
};
use actix_web::web::Data;
use mockall::predicate::*;
use mockall::*;
use prisma_client_rust::raw;
use rand::Rng;

use crate::services::tasks::task_kind::TaskKind;
use crate::services::traits::async_task::TaskError;
use uuid::Uuid;

use lazy_static::lazy_static;

static INIT_RUN: Once = Once::new();

// Mock definition
mock! {
    pub TaskScheduler {
        pub fn schedule(&self, task: TaskKind) -> Result<Uuid, TaskError>;
    }
}

impl TaskScheduler for MockTaskScheduler {
    fn schedule(&self, task: TaskKind) -> Result<Uuid, TaskError> {
        self.schedule(task)
    }

    fn get_task(&self, task_id: Uuid) -> Option<TaskKind> {
        Some(TaskKind::Exploration(ExploreAction::new(
            String::from("hero_id"),
            RegionName::Dusane,
        )))
    }
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
        .args(&["prisma", "migrate", "dev", "--skip-generate"])
        .env("DATABASE_URL", format!("{}/testdb", URL.to_string())) // make sure to use the new database name
        .output()
        .expect("failed to execute process");

    println!("migration complete output: {:?} \n", _output);

    Ok(())
}

pub async fn setup_test_database() -> Result<Data<PrismaClient>, Box<dyn std::error::Error>> {
    INIT_RUN.call_once(|| {
        let _ = setup_once();
    });
    // Re-initialize connected to test DB
    let prisma_client = PrismaClient::_builder()
        .with_url(format!("{}/testdb", URL.to_string())) // make sure to use the new database name
        .build()
        .await?;

    Ok(Data::new(prisma_client))
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
    )
}

//  example

// ._execute_raw(raw!(
//         "INSERT INTO Post (published, title) VALUES ({}, {})",
//         PrismaValue::Boolean(false),
//         PrismaValue::String("A Title".to_string())
//     ))
