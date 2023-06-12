use std::sync::Arc;

use crate::services::impls::hero_service::ServiceHeroes;
use crate::services::impls::region_service::RegionServiceImpl;
use crate::services::traits::hero_service::HeroService;
use crate::services::traits::region::RegionService;
use crate::test_helpers::{random_hero, setup_test_database, MockTaskScheduler};
use crate::{models::region::RegionName, services::tasks::explore::ExploreAction};
use uuid::Uuid;

#[tokio::test]
async fn test_start_exploration() {
    let mut mock_scheduler = MockTaskScheduler::new();
    mock_scheduler
        .expect_schedule()
        .returning(|_| Ok(Uuid::new_v4()));

    let prisma_client = setup_test_database().await.unwrap();
    let service = RegionServiceImpl::new(Arc::new(mock_scheduler), prisma_client.clone());

    let hero_id = "test_hero_id".to_string();
    let region_name = RegionName::Dusane; // assuming this is a valid region name

    // Execute the start_exploration function and get the result
    let result = service.start_exploration(hero_id, region_name);

    // Assert that the result is an Ok value (exploration start was successful)
    assert!(result.is_ok(), "Starting exploration failed");
}

#[tokio::test]
async fn test_generate_result_for_exploration() {
    let mock_scheduler = MockTaskScheduler::new();

    let prisma_client = setup_test_database().await.unwrap();
    let service = RegionServiceImpl::new(Arc::new(mock_scheduler), prisma_client.clone());

    let hero_service = ServiceHeroes::new(prisma_client.clone());
    let mut new_hero = random_hero();
    new_hero.attributes.exploration = 17;
    let hero = hero_service.create_hero(new_hero).await.unwrap();
    let hero_id = hero.get_id().clone();
    let region_name = RegionName::Dusane; // assuming this is a valid region name

    let explore_action = ExploreAction::new(hero.get_id(), region_name);

    // Execute the generate_result_for_exploration function and get the result
    let result = service
        .generate_result_for_exploration(&explore_action)
        .await;

    // Assert that the result is an Ok value (generation of result was successful)
    assert!(result.is_ok(), "Generation of exploration result failed");

    // Optionally, you could assert the returned RegionActionResult fields
    if let Ok(region_action_result) = &result {
        assert!(region_action_result.xp > explore_action.xp, "XP mismatch");
        assert!(
            region_action_result.discovery_level_increase > explore_action.discovery_level as f64,
            "Discovery level increase mismatch"
        );
        assert_eq!(
            region_action_result.hero_id,
            hero.get_id(),
            "Hero ID mismatch"
        );
        assert!(
            region_action_result.resources.is_empty(),
            "Resources should be empty"
        );
    }

    // Test the results_by_hero function
    let results_by_hero = service.results_by_hero(hero_id.clone()).await.unwrap();
    // Assert that at least one result is retrieved
    assert!(!results_by_hero.is_empty(), "No results found for hero");

    // Optionally, check if the result retrieved matches the result generated above
    let matching_results = results_by_hero
        .iter()
        .filter(|res| res.hero_id == hero_id && res.xp == result.as_ref().unwrap().xp)
        .count();
    assert!(
        matching_results > 0,
        "Matching result not found in results_by_hero"
    );
}
