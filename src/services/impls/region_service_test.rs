use std::collections::HashMap;
use std::fmt::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use prisma_client_rust::chrono::Duration;

use crate::models::hero::Hero;
use crate::models::task::TaskLootBox::Region;
use crate::models::task::{RegionActionResult, TaskKind};
use crate::prisma::PrismaClient;
use crate::repos::region_repo::RegionRepo;
use crate::services::impls::action_executor::ActionExecutor;
use crate::services::impls::hero_service::ServiceHeroes;
use crate::services::traits::async_task::{Task, TaskStatus};
use crate::services::traits::hero_service::HeroService;
use crate::test_helpers::{random_hero, setup_test_database};
use crate::{models::region::RegionName, services::tasks::explore::ExploreAction};

//TODO: extract these setups to one func
#[tokio::test]
async fn test_start_exploration() {
    let prisma_client = setup_test_database().await.unwrap();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::seconds(10));
    let prisma_clone = prisma_client.clone().into_inner();
    let game_engine = Arc::new(ActionExecutor::new(prisma_clone));

    let hero = random_hero();
    let region_name = RegionName::Dusane; // assuming this is a valid region name

    // Execute the start_exploration function and get the result
    let task = ExploreAction::new(hero, region_name, &durations).unwrap();
    let sent = game_engine.task_sender.send(TaskKind::Exploration(task));

    // Assert that the result is an Ok value (exploration start was successful)
    assert!(sent.is_ok(), "Starting exploration failed");
}

#[tokio::test]
async fn test_start_exploration_task_status() {
    let prisma_client = setup_test_database().await.unwrap();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::seconds(3));
    let prisma_clone = prisma_client.clone().into_inner();
    let game_engine = ActionExecutor::new(prisma_clone.clone());

    let hero_service = ServiceHeroes::new(prisma_clone.clone());
    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = hero_service.create_hero(new_hero).await.unwrap();
    let hero_id = hero.get_id().clone();

    let region_name = RegionName::Dusane;
    // Execute the start_exploration function and get the result
    let task = ExploreAction::new(hero, region_name.clone(), &durations).unwrap();
    let _ = game_engine.task_sender.send(TaskKind::Exploration(task));

    tokio::time::sleep(Duration::milliseconds(300).to_std().unwrap()).await;
    let task = game_engine.scheduler.get_current_task(hero_id.as_str());
    assert!(task.is_some(), "Task not found");

    let explore_task = match task {
        Some(task_kind) => match task_kind {
            TaskKind::Exploration(explore_task) => explore_task,
            _ => panic!("TaskKind is not Exploration"),
        },
        None => todo!(),
    };

    assert_eq!(hero_id, explore_task.hero_id());
    assert_eq!(region_name, explore_task.region_name);

    // Check task status
    assert_eq!(explore_task.check_status(), TaskStatus::InProgress);
    //wait 2s and check status again
    tokio::time::sleep(Duration::seconds(3).to_std().unwrap()).await;
    assert_eq!(explore_task.check_status(), TaskStatus::Completed);
}

async fn delay(time: Duration) {
    tokio::time::sleep(time.to_std().unwrap()).await;
}

async fn random_hero_and_explore() -> (
    Hero,
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    Arc<ActionExecutor>,
    Arc<PrismaClient>,
) {
    let prisma_client = setup_test_database().await.unwrap();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::milliseconds(500));
    let prisma_clone = prisma_client.clone().into_inner();
    let game_engine = ActionExecutor::new(prisma_clone.clone());

    let hero_service = ServiceHeroes::new(prisma_clone.clone());

    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = hero_service.create_hero(new_hero).await.unwrap();

    let hero_clone = hero.clone();
    let game_engine_clone = game_engine.clone();
    // Create a boxed future for the run_the_task logic
    let run_the_task: Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>> =
        Box::new(move || {
            Box::pin(async move {
                let region_name = RegionName::Dusane;
                let task = ExploreAction::new(hero_clone.clone(), region_name.clone(), &durations)
                    .unwrap();
                let _ = game_engine_clone
                    .task_sender
                    .send(TaskKind::Exploration(task));
                tokio::time::sleep(Duration::milliseconds(600).to_std().unwrap()).await;
            })
        });

    (hero, run_the_task, game_engine, prisma_clone.clone())
}

#[tokio::test]
async fn test_generate_result_for_exploration() {
    let prisma_client = setup_test_database().await.unwrap();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::milliseconds(500));
    let prisma_clone = prisma_client.clone().into_inner();
    let game_engine = ActionExecutor::new(prisma_clone.clone());

    let hero_service = ServiceHeroes::new(prisma_clone.clone());
    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = hero_service.create_hero(new_hero).await.unwrap();
    let hero_id = hero.get_id().clone();

    let region_name = RegionName::Dusane;
    // Execute the start_exploration function and get the result
    let task = ExploreAction::new(hero.clone(), region_name.clone(), &durations).unwrap();
    let _ = game_engine.task_sender.send(TaskKind::Exploration(task));
    tokio::time::sleep(Duration::milliseconds(100).to_std().unwrap()).await;

    let result: Result<RegionActionResult, Error> =
        match game_engine.result_broadcast_receiver.recv_async().await {
            Ok(res) => {
                // unpack the TaskLootBox enum into Region(RegionActionResult)
                let region_action_result = match res {
                    Region(region_action_result) => region_action_result,
                    // return regular error
                    _ => panic!("Result is not Region"),
                };
                assert!(region_action_result.xp > 0, "XP should be greater than 0");
                assert!(
                    region_action_result.discovery_level_increase > 0.0,
                    "Discovery level increase should be greater than 0"
                );
                assert_eq!(
                    region_action_result.hero_id,
                    hero.get_id(),
                    "Hero ID mismatch"
                );
                Ok(region_action_result)
            }
            Err(_) => panic!("No result received"),
        };
    assert!(result.is_ok(), "Generation of exporation result failed");

    let hero_service = ServiceHeroes::new(prisma_clone.clone());
    let hero = hero_service.get_hero(hero_id).await.unwrap();
    println!("Hero stamina new: {:?}", hero.stamina);
    assert!(hero.stamina < 100, "Stamina should be less than 100");
}

#[tokio::test]
async fn test_hero_status_after_explore() {
    let (hero, run_the_task, _, prisma_client) = random_hero_and_explore().await;
    // RegionHero Before
    let region_repo = RegionRepo::new(prisma_client.clone());
    let current_region_hero = region_repo
        .get_current_hero_region(hero.get_id().as_ref())
        .await
        .unwrap();

    run_the_task().await;

    let hero_service = ServiceHeroes::new(prisma_client.clone());
    let mut updated_hero = hero_service.get_hero(hero.get_id()).await.unwrap();
    let latest_action = hero_service
        .latest_action_results(hero.get_id())
        .await
        .unwrap()
        .iter()
        .cloned()
        .find(|action| action.hero_id == hero.get_id());
    let before_stamina = updated_hero.stamina;
    delay(Duration::seconds(3)).await;
    let latest_action_result = latest_action.clone().unwrap();
    updated_hero.regenerate_stamina(latest_action_result);
    assert!(
        updated_hero.stamina > before_stamina,
        "Stamina should be greater than 0"
    );
    //Regionhero after
    //
    let after_region_hero = region_repo
        .get_current_hero_region(hero.get_id().as_ref())
        .await
        .unwrap();

    println!(
        "before region hero discovery {:?} and after {:?}",
        current_region_hero.discovery_level, after_region_hero.discovery_level
    );
    assert!(after_region_hero.discovery_level > current_region_hero.discovery_level);
}
