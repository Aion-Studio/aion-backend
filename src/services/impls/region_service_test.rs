use std::collections::HashMap;
use std::fmt::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::configuration::{ChannelDurations, ExploreDurations};
use prisma_client_rust::chrono::Duration;

use crate::events::initialize::initialize_handlers;
use crate::handlers::heroes::get_hero_status;
use crate::handlers::regions::{do_channel, do_explore};
use crate::infra::Infra;
use crate::models::hero::Hero;
use crate::models::region::RegionName;
use crate::models::resources::Resource;
use crate::prisma::PrismaClient;
use crate::repos::region_repo::Repo;
use crate::test_helpers::{random_hero, setup_test_database};

async fn delay(time: Duration) {
    tokio::time::sleep(time.to_std().unwrap()).await;
}

async fn random_hero_and_explore() -> (
    Hero,
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    Arc<PrismaClient>,
) {
    let prisma_client = setup_test_database().await.unwrap();
    Infra::initialize(prisma_client.clone().into_inner());
    initialize_handlers();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::milliseconds(500));
    let prisma_clone = prisma_client.clone().into_inner();

    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = Infra::repo().insert_hero(new_hero).await.unwrap();
    let hero_clone = hero.clone();
    // Create a boxed future for the run_the_task logic
    let run_the_task: Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>> =
        Box::new(move || {
            Box::pin(async move {
                let region_name = RegionName::Dusane;
                do_explore(&hero_clone, &region_name, &ExploreDurations(durations)).unwrap();
                tokio::time::sleep(Duration::milliseconds(800).to_std().unwrap()).await;
            })
        });

    (hero, run_the_task, prisma_clone.clone())
}

async fn explore(hero: Hero, region_name: RegionName) -> Result<(), Error> {
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::milliseconds(500));
    let wrap = ExploreDurations(durations);
    do_explore(&hero, &region_name, &wrap).unwrap();
    tokio::time::sleep(Duration::milliseconds(650).to_std().unwrap()).await;
    Ok(())
}

#[tokio::test]
async fn test_generate_result_for_exploration() {
    let prisma_client = setup_test_database().await.unwrap();
    Infra::initialize(prisma_client.clone().into_inner());
    initialize_handlers();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::milliseconds(500));

    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = Infra::repo().insert_hero(new_hero).await.unwrap();
    let hero_id = hero.get_id().clone();

    let region_name = RegionName::Dusane;
    // Execute the start_exploration function and get the result
    do_explore(&hero, &region_name, &ExploreDurations(durations)).unwrap();
    tokio::time::sleep(Duration::milliseconds(800).to_std().unwrap()).await;

    let hero = Infra::repo().get_hero(hero_id).await.unwrap();
    assert!(hero.stamina < 100, "Stamina should be less than 100");
}

#[tokio::test]
async fn test_hero_status_after_explore() {
    let (hero, run_the_task, prisma_client) = random_hero_and_explore().await;

    // RegionHero Before
    let region_repo = Repo::new(prisma_client.clone());
    let current_region_hero = region_repo
        .get_current_hero_region(hero.get_id().as_ref())
        .await
        .unwrap();
    run_the_task().await;

    let before_stamina = Infra::repo().get_hero(hero.get_id()).await.unwrap().stamina;
    println!("waiting 3 seconds...");
    delay(Duration::seconds(3)).await;
    let updated_hero = Infra::repo().get_hero(hero.get_id()).await.unwrap();

    assert!(
        updated_hero.stamina > before_stamina,
        "Stamina should be greater than 0"
    );

    let after_region_hero = region_repo
        .get_current_hero_region(hero.get_id().as_ref())
        .await
        .unwrap();
    println!("AFTER REGION HERO {:?}", after_region_hero);

    assert!(after_region_hero.discovery_level > current_region_hero.discovery_level);
}

#[tokio::test]
async fn test_leyline_increased_visible() {
    let (hero, run_the_task, _) = random_hero_and_explore().await;

    let updated_hero = Infra::repo().get_hero(hero.get_id()).await.unwrap();
    let mut status = get_hero_status(updated_hero.clone()).await.unwrap();

    run_the_task().await;
    loop {
        if status.available_leylines.len() > 4 {
            break;
        }

        println!(
            "LOOP:: status available leylines {:?}",
            status.available_leylines.len()
        );

        let _ = explore(hero.clone(), RegionName::Dusane).await;
        let updated_hero = Infra::repo().get_hero(hero.get_id()).await.unwrap();
        status = get_hero_status(updated_hero).await.unwrap();
    }
    // has to regen stamina using get_hero
    assert!(status.available_leylines.len() > 1);
    println!("hero status at end {:?}", status);
}

#[tokio::test]
async fn test_channeling_leyline() {
    let leyline_name = "Dusarock".to_string();
    //alias hero as hero_before
    let (hero_before, _, _) = random_hero_and_explore().await;
    let mut leyline_durations = HashMap::new();
    leyline_durations.insert(leyline_name.clone(), Duration::milliseconds(500));

    let durations = ChannelDurations(leyline_durations);
    let _ = do_channel(&hero_before, &leyline_name, &durations).unwrap();
    delay(Duration::milliseconds(800)).await;
    let hero = Infra::repo().get_hero(hero_before.get_id()).await.unwrap();

    assert!(hero.base_stats.xp > hero_before.base_stats.xp);
    assert!(hero.resources.get(&Resource::Aion) > hero_before.resources.get(&Resource::Aion));
}
