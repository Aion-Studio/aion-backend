use aion_server::create_test_hero;
use aion_server::events::combat::CombatantState;
use aion_server::models::cards::Card;
use aion_server::models::combatant::Combatant;
use aion_server::models::npc::Monster;
use aion_server::tests::combat_context::{
    create_attack_hero, create_poison_hero, CombatTestContext,
};

#[tokio::test]
async fn test_combat_poision_effect() {
    let hero = create_poison_hero(); // adds 2 poison for 2 rounds
    let monster = Monster::default();
    let context = CombatTestContext::new(hero, monster).await;

    let monster_combatant = context.get_monster().await.unwrap();
    let initial_monster_hp = monster_combatant.get_hp();

    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();
    context.end_turn_npc().await.unwrap();

    let encounter_state = context.get_encounter_state().await.unwrap();

    if let CombatantState::Npc { hp, .. } = encounter_state.npc_state {
        assert_eq!(initial_monster_hp - 2, hp);
    } else {
        panic!("Expected NPC state");
    }

    println!("Round 2");
    context.end_turn().await.unwrap();
    context.end_turn_npc().await.expect("Failed to end turn");

    match context.get_encounter_state().await.unwrap().npc_state {
        CombatantState::Npc { hp, .. } => {
            assert_eq!(initial_monster_hp - 4, hp);
        }
        _ => panic!("Expected NPC state"),
    }

    // Clean up the encounter
}

#[tokio::test]
async fn test_combat_heal_effect() {
    let context =
        CombatTestContext::new(create_test_hero!(Card::heal(3)), Monster::default()).await;
    let mut hero = context.get_hero().await.unwrap();
    println!("Initial HP: {}", hero.get_hp());
    hero.take_damage(10);
    if let CombatantState::Player { hp, .. } =
        context.get_encounter_state().await.unwrap().player_state
    {
        print!("hp encounter {:?}", hp);
    }
    let initial_hp = hero.get_hp();
    context.play_card(0).await.unwrap();
    let encounter_state = context.get_encounter_state().await.unwrap();

    if let CombatantState::Player { hp, .. } = encounter_state.player_state {
        println!("HP: {}", hp);
        assert_eq!(initial_hp + 3, hp);
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_combat_attack_effect() {
    let context = CombatTestContext::new(create_attack_hero(), Monster::default()).await;
    let monster_combatant = context.get_monster().await.unwrap();
    let initial_monster_hp = monster_combatant.get_hp();
    let mut hero = context.get_hero().await.unwrap();
    let damage_before = hero.get_hp();
    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();
    context.end_turn_npc().await.unwrap();
    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Npc { hp, .. } = encounter_state.npc_state {
        assert_eq!(initial_monster_hp - 10, hp);
    } else {
        panic!("Expected NPC state");
    }
}
