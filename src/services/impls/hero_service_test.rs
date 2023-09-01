use crate::models::hero::{Attributes, BaseStats, Hero, Range};
use crate::services::impls::hero_service::ServiceHeroes;
use crate::services::traits::hero_service::HeroService;
use crate::test_helpers::{random_hero, setup_test_database};

#[tokio::test]
async fn test_create_hero() {
    let prisma_client = setup_test_database().await.unwrap();
    let service = ServiceHeroes::new(prisma_client.into_inner());

    let new_hero = Hero::new(
        BaseStats {
            id: None,
            level: 1,
            xp: 0,
            damage: Range { min: 1, max: 10 },
            hit_points: 100,
            mana: 50,
            armor: 10,
        },
        Attributes {
            id: None,
            strength: 10,
            resilience: 10,
            agility: 10,
            intelligence: 10,
            exploration: 10,
            crafting: 10,
        },
        100,
        0,
    );

    // Execute the create_hero function and get the result
    let result = service.create_hero(new_hero).await;

    // Assert that the result is an Ok value (creation was successful)
    assert!(result.is_ok(), "Hero creation failed");

    // Optionally, you could assert the returned Hero fields
    if let Ok(hero) = result {
        assert_eq!(hero.aion_capacity, 100, "Hero aion_capacity mismatch");
        assert_eq!(hero.aion_collected, 0, "Hero aion_collected mismatch");
        // Continue for other fields...
    }
}

#[tokio::test]
async fn test_level_up_hero() {
    let prisma_client = setup_test_database().await.unwrap();
    let service = ServiceHeroes::new(prisma_client.into_inner());

    let new_hero = random_hero();
    // First create a hero
    let create_result = service.create_hero(new_hero).await;
    assert!(create_result.is_ok(), "Initial Hero creation failed");

    let hero = create_result.unwrap();
    // Level up the hero
    let result = service.level_up_hero(hero.clone()).await;

    // Assert that the result is an Ok value (level up was successful)
    assert!(result.is_ok(), "Hero level up failed");

    // Optionally, you could assert that the hero's level has been increased by 1
    if let Ok(updated_hero) = result {
        assert_eq!(
            updated_hero.base_stats.level,
            hero.base_stats.level + 1,
            "Hero level update mismatch"
        );
    }
}
