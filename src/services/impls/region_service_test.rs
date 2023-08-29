use std::collections::HashMap;
use std::sync::Arc;

use prisma_client_rust::chrono::Duration;

use crate::models::task::TaskKind;
use crate::services::impls::hero_service::ServiceHeroes;
use crate::services::impls::tasks::TaskManager;
use crate::services::traits::async_task::{Task, TaskStatus};
use crate::services::traits::hero_service::HeroService;
use crate::test_helpers::{random_hero, setup_test_database};
use crate::{models::region::RegionName, services::tasks::explore::ExploreAction};
use crate::services::impls::action_executor::ActionExecutor;
use crate::services::impls::region_service::RegionService;

//TODO: extract these setups to one func
#[tokio::test]
async fn test_start_exploration() {
    let prisma_client = setup_test_database().await.unwrap();
    let mut durations = HashMap::new();
    durations.insert(RegionName::Dusane, Duration::seconds(10));
    let prisma_clone = prisma_client.clone().into_inner();
    let game_engine = Arc::new(ActionExecutor::new(prisma_clone));


    let hero_id = "test_hero_id".to_string();
    let region_name = RegionName::Dusane; // assuming this is a valid region name

    // Execute the start_exploration function and get the result
    let task = ExploreAction::new(hero_id, region_name, &durations);
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
    let game_engine = ActionExecutor::new(prisma_clone);

    println!("ActionExecutor ID in test: {:?}", Arc::as_ptr(&game_engine));

    let hero_service = ServiceHeroes::new(prisma_client.clone());
    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = hero_service.create_hero(new_hero).await.unwrap();
    let hero_id = hero.get_id().clone();

    let region_name = RegionName::Dusane;
    // Execute the start_exploration function and get the result
    let task = ExploreAction::new(hero_id.clone(), region_name.clone(), &durations);
    let sent = game_engine.task_sender.send(TaskKind::Exploration(task));

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
    println!("Task status in progress is correct");
    //wait 2s and check status again
    tokio::time::sleep(Duration::seconds(3).to_std().unwrap()).await;
    assert_eq!(explore_task.check_status(), TaskStatus::Completed);
    println!("Task status completed is correct");
}
//
// #[tokio::test]
// async fn test_generate_result_for_exploration() {
//     let prisma_client = setup_test_database().await.unwrap();
//     let scheduler = Arc::new(TaskManager::new());
//     let mut durations = HashMap::new();
//     durations.insert(RegionName::Dusane, Duration::seconds(10));
//     let game_engine = Arc::new(ActionExecutor::new(prisma_client.clone()));
//     let tx = game_engine.clone().result_channels().unwrap();
//
//     let service = RegionService::new(scheduler, prisma_client.clone(), durations.clone(), tx);
//
//     let hero_service = ServiceHeroes::new(prisma_client.clone());
//     let mut new_hero = random_hero();
//     new_hero.attributes.exploration = 17;
//     let hero = hero_service.create_hero(new_hero).await.unwrap();
//     let hero_id = hero.get_id().clone();
//     let region_name = RegionName::Dusane; // assuming this is a valid region name
//
//     let explore_action = ExploreAction::new(hero.get_id(), region_name, &durations);
//
//     // Execute the generate_result_for_exploration function and get the result
//     let result = service
//         .generate_result_for_exploration(&explore_action)
//         .await;
//
//     // Assert that the result is an Ok value (generation of result was successful)
//     assert!(result.is_ok(), "Generation of exploration result failed");
//
//     // Optionally, you could assert the returned RegionActionResult fields
//     if let Ok(region_action_result) = &result {
//         assert!(region_action_result.xp > explore_action.xp, "XP mismatch");
//         assert!(
//             region_action_result.discovery_level_increase > explore_action.discovery_level as f64,
//             "Discovery level increase mismatch"
//         );
//         assert_eq!(
//             region_action_result.hero_id,
//             hero.get_id(),
//             "Hero ID mismatch"
//         );
//         assert!(
//             region_action_result.resources.is_empty(),
//             "Resources should be empty"
//         );
//     }
//
//     // Test the results_by_hero function
//     let results_by_hero = service.results_by_hero(hero_id.clone()).await.unwrap();
//     // Assert that at least one result is retrieved
//     assert!(!results_by_hero.is_empty(), "No results found for hero");
//
//     // Optionally, check if the result retrieved matches the result generated above
//     let matching_results = results_by_hero
//         .iter()
//         .filter(|res| res.hero_id == hero_id && res.xp == result.as_ref().unwrap().xp)
//         .count();
//     assert!(
//         matching_results > 0,
//         "Matching result not found in results_by_hero"
//     );
// }
