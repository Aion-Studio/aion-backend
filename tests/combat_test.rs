use aion_server::create_test_hero;
use aion_server::events::combat::{CombatantIndex, CombatantState};
use aion_server::models::cards::{Card, Effect};
use aion_server::models::combatant::{Combatant, CombatantType};
use aion_server::models::npc::Monster;
use aion_server::models::talent::{CombatModifier, Spell};
use aion_server::prisma::{CardType, DamageType};
use aion_server::services::impls::combat_controller::CombatCommand;
use aion_server::tests::combat_context::{
    create_attack_hero, create_chaos_attack_hero, create_poison_hero, CombatTestContext,
};

use CombatCommand::*;

async fn get_initiative_modifier(context: &CombatTestContext) -> i32 {
    let enc = context.get_encounter().await;
    let modifiers = enc.get_modifiers();
    let hero_id = context.get_hero().await.unwrap().get_id();
    modifiers
        .get(&hero_id)
        .and_then(|modifier| {
            modifier.iter().find_map(|m| {
                if let CombatModifier::Initiative(i) = m {
                    Some(*i)
                } else {
                    None
                }
            })
        })
        .unwrap_or(0)
}

#[tokio::test]
async fn test_combat_poision_effect() {
    let hero = create_poison_hero(); // adds 2 poison for 2 rounds
    let monster = Monster::default();
    let context = CombatTestContext::new(hero, monster).await;

    let monster_combatant = context.get_monster().await.unwrap();
    let initial_monster_hp = monster_combatant.get_hp();
    println!("Initial monster hp: {}", initial_monster_hp);

    context.play_card(0).await.unwrap();

    let encounter_state = context.get_encounter_state().await.unwrap();

    println!("Initial monster hp2: {}", initial_monster_hp);

    if let CombatantState::Npc { hp, .. } = encounter_state.npc_state {
        assert_eq!(
            initial_monster_hp - 2,
            hp,
            "expect damage from poision applied instantly"
        );
    } else {
        panic!("Expected NPC state");
    }

    context.end_turn().await.unwrap();
    context.end_turn_npc().await.expect("Failed to end turn");

    match context.get_encounter_state().await.unwrap().npc_state {
        CombatantState::Npc { hp, .. } => {
            assert_eq!(
                initial_monster_hp - 4,
                hp,
                "expected poison number of rounds applied to be correct"
            );
        }
        _ => panic!("Expected NPC state"),
    }

    context.end_turn().await.unwrap();
    context.end_turn_npc().await.expect("Failed to end turn");

    match context.get_encounter_state().await.unwrap().npc_state {
        CombatantState::Npc { hp, .. } => {
            assert_eq!(
                initial_monster_hp - 4,
                hp,
                "expected poison number of rounds applied to be correct"
            );
        }
        _ => panic!("Expected NPC state"),
    }

    // Clean up the encounter
}

#[tokio::test]
async fn test_combat_heal_effect() {
    let context =
        CombatTestContext::new(create_test_hero!(Card::heal(3)), Monster::default()).await;
    let mut encounter = context.get_encounter().await;

    let hero_combatant = encounter.get_player_combatant();
    let initial_hp = hero_combatant.get_hp(); // 1. Save the starting HP
    hero_combatant.take_damage(10, true); // 2. Simulate taking damage
    context.sync_encounter(encounter).await.unwrap(); //3. Sync the encounter to Redis

    context.play_card(0).await.unwrap(); // 4. Play the heal card
                                         //
    let encounter_state = context.get_encounter_state().await.unwrap();

    if let CombatantState::Player { hp, .. } = encounter_state.player_state {
        println!("HP: {}", hp);
        assert_eq!(initial_hp - 10 + 3, hp);
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_combat_normal_attack_effect() {
    let context = CombatTestContext::new(create_attack_hero(5), Monster::default()).await;

    let mut encounter = context.get_encounter().await;
    let npc = encounter.get_monster();
    let initial_monster_hp = npc.get_hp();

    context.play_card(0).await.unwrap();
    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Npc { hp, .. } = encounter_state.npc_state {
        assert_eq!(initial_monster_hp - 5 + npc.get_armor(), hp);
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_mana_spent() {
    let context = CombatTestContext::new(create_attack_hero(5), Monster::default()).await; // attack
                                                                                           // card mana cost is 1

    let mut encounter = context.get_encounter().await;
    let hero = encounter.get_player_combatant();
    let initial_mana = hero.get_mana();

    context.play_card(0).await.unwrap();
    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Player { mana, .. } = encounter_state.player_state {
        assert_eq!(initial_mana - 1, mana);
    } else {
        panic!("Expected Player state");
    }

    context.play_card(0).await.unwrap();
}

#[tokio::test]
async fn test_card_played_discard() {
    let context = CombatTestContext::new(create_attack_hero(5), Monster::default()).await; // attack

    let id = context.play_card(0).await.unwrap();
    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Player {
        cards_in_discard, ..
    } = encounter_state.player_state
    {
        let discard_card_id = cards_in_discard[0].id.clone();
        assert_eq!(discard_card_id, id);
    } else {
        panic!("Expected Player state");
    }
}

#[tokio::test]
async fn test_not_enough_mana() {
    let context = CombatTestContext::new(create_attack_hero(5), Monster::default()).await; // attack

    let mut encounter = context.get_encounter().await;
    let hero = encounter.get_player_combatant();

    let _ = context.play_card(0).await;
    let _ = context.play_card(0).await;
    let result = context.play_card(0).await;
    assert!(result.is_ok());
    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Player { mana, .. } = encounter_state.player_state {
        assert_eq!(0, mana);
    } else {
        panic!("Expected Player state");
    }

    let result = context.play_card(0).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_chaos_attack_on_hero() {
    let context =
        CombatTestContext::new(create_chaos_attack_hero(), Monster::with_fireball(25)).await;

    let mut encounter = context.get_encounter().await;
    let hp_before = encounter.get_player_combatant().get_hp();
    let npc = encounter.get_monster();
    // npc.spells.append(&mut vec![Spell::fireball(25)]);

    context.end_turn().await.unwrap();
    let spell = npc.spells.first().expect("No spell").clone();

    context.npc_turn(UseSpell(spell)).await.unwrap();

    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Player { hp, .. } = encounter_state.player_state {
        assert_eq!(hp_before - 25, hp);
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_hero_armor_effect() {
    let context = CombatTestContext::new(create_attack_hero(5), Monster::default()).await;
    let mut encounter = context.get_encounter().await;
    let hero = encounter.get_player_combatant();
    let initial_armor = hero.get_armor();
    let initial_hp = hero.get_hp();

    context.end_turn().await.unwrap();

    context
        .npc_turn(
            UseSpell(Spell::bolt(20)), // bolt spell does 20 damage
        )
        .await
        .unwrap();

    let encounter_state = context.get_encounter_state().await.unwrap();
    if let CombatantState::Player { hp, armor, .. } = encounter_state.player_state {
        assert_eq!(
            initial_hp + initial_armor - 20,
            hp,
            "Armor should absorb damage"
        );
        assert_eq!(0, armor, "Armor should be spent");
    } else {
        panic!("Expected Player state");
    }

    let player_hp = match encounter_state.player_state {
        CombatantState::Player { hp, .. } => hp,
        _ => panic!("Expected Player state"),
    };

    context.npc_turn(UseSpell(Spell::bolt(20))).await.unwrap();
    let encounter_state = context.get_encounter_state().await.unwrap();

    if let CombatantState::Player { hp, .. } = encounter_state.player_state {
        assert_eq!(
            player_hp - 20,
            hp,
            "Hp should reduce by spell dmg amount after armor is spent"
        );
    } else {
        panic!("Expected Player state");
    }
}

#[tokio::test]
async fn test_buff_damage() {
    use DamageType::*;
    let buff_card = Card::buff_damage(2, None);
    let mut cards: Vec<Card> = (0..5).map(|_| Card::attack(5, Normal)).collect();
    cards.append(
        (0..7)
            .map(|_| buff_card.clone())
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await;
    let npc = context.npc().await;
    let hp_before = npc.get_hp();
    let buff_card = match context
        .find_card_recursive(|e| matches!(e, Effect::BuffDamage { .. }))
        .await
    {
        Some(c) => c,
        None => {
            panic!("Expected buff card");
        }
    };

    //add the buff
    context.play_card_obj(buff_card).await.unwrap();
    //play the ataack card
    let attack_card = match context
        .find_card_recursive(|e| matches!(e, Effect::Damage { .. }))
        .await
    {
        Some(c) => c,
        None => {
            panic!("Expected attack card");
        }
    };

    context.play_card_obj(attack_card).await.unwrap();

    if let CombatantState::Npc { hp, .. } = context.get_encounter_state().await.unwrap().npc_state {
        assert_eq!(hp, hp_before - (5 + 2 - npc.get_armor()));
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_debuff_damage() {
    let debuff_card = Card::debuff_damage(2);
    let mut cards: Vec<Card> = vec![];
    cards.append(
        (0..12)
            .map(|_| debuff_card.clone())
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await;
    let hero = context.get_hero().await.unwrap();
    let hp_before = hero.get_hp();
    //add the debuff
    context.play_card(0).await.unwrap();

    context.end_turn().await.unwrap();

    context.npc_turn(UseSpell(Spell::bolt(10))).await.unwrap();

    if let CombatantState::Player { hp, .. } =
        context.get_encounter_state().await.unwrap().player_state
    {
        assert_eq!(hp, hp_before - (10 - 2 + hero.get_armor()));
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_initative() {
    let cards: Vec<Card> = (0..12).map(|_| Card::initiative(1)).collect();
    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await;

    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();

    assert_eq!(
        context.get_encounter().await.whos_turn(),
        CombatantIndex::Npc
    );
    context.end_turn_npc().await.expect("Failed to end turn");

    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();
    assert_eq!(
        context.get_encounter().await.whos_turn(),
        CombatantIndex::Npc
    );

    context.end_turn_npc().await.expect("Failed to end turn");
    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();
    assert_eq!(
        context.get_encounter().await.whos_turn(),
        CombatantIndex::Player,
        "Expected NPC to miss turn after 3rd initiative applied"
    );
    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();
    assert_eq!(
        context.get_encounter().await.whos_turn(),
        CombatantIndex::Npc,
        "Initiative should have reset after 3"
    );
}

#[tokio::test]
async fn test_initative_remove() {
    let cards: Vec<Card> = (0..12).map(|_| Card::initiative_remove(1)).collect();
    /*
     * NPC puts 2 initiative on hero and we ensure that number is there first
     *
     * */
    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await;
    context.end_turn().await.unwrap();
    context
        .npc_turn(UseSpell(Spell::initiative(2)))
        .await
        .unwrap();

    assert_eq!(get_initiative_modifier(&context).await, 2);

    context.end_turn_npc().await.expect("Failed to end turn");

    /* Player removes a pt of initiative with a card */
    context.play_card(0).await.unwrap();

    assert_eq!(get_initiative_modifier(&context).await, 1);
}

#[tokio::test]
async fn test_buff_armor() {
    let buff_card = Card::buff_armor(2);
    let mut cards: Vec<Card> = vec![];
    cards.append(
        (0..12)
            .map(|_| buff_card.clone())
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await; // starts
    let hero = context.get_hero().await.unwrap();
    // with 1 armor
    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();

    let mut encounter = context.get_encounter().await;

    context.npc_turn(UseSpell(Spell::bolt(10))).await.unwrap();

    if let CombatantState::Player { hp, .. } =
        context.get_encounter_state().await.unwrap().player_state
    {
        assert_eq!(hp, hero.get_hp() - (10 - hero.get_armor() - 2));
    } else {
        panic!("Expected NPC state");
    }
    let hero_armor_after = context.get_hero().await.unwrap().get_armor();
    assert_eq!(0, hero_armor_after);
}

#[tokio::test]
async fn test_debuff_armor() {
    let debuff_card = Card::debuff_armor(5);
    let mut cards: Vec<Card> = vec![];
    cards.append(
        (0..6)
            .map(|_| debuff_card.clone())
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    cards.append(
        (0..6)
            .map(|_| Card::attack(5, DamageType::Normal))
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await; //monster
                                                                                              //starts with 2 armor
    let npc_hp_before = context.npc().await.get_hp();
    //add the debuff
    context.end_turn().await.unwrap();

    context
        .npc_turn(
            UseSpell(Spell::buff_armor(1)), //add some buff armor to npc
        )
        .await
        .unwrap();

    context.end_turn_npc().await.expect("Failed to end turn");

    let debuff_card = match context
        .find_card_recursive(|e| matches!(e, Effect::DebuffArmor { .. }))
        .await
    {
        Some(c) => c,
        None => {
            panic!("Expected debuff card");
        }
    };

    context.play_card_obj(debuff_card).await.unwrap();
    let attack_card = match context
        .find_card_recursive(|e| matches!(e, Effect::Damage { .. }))
        .await
    {
        Some(c) => c,
        None => {
            panic!("Expected attack card");
        }
    };

    context.play_card_obj(attack_card).await.unwrap();

    if let CombatantState::Npc { hp, .. } = context.get_encounter_state().await.unwrap().npc_state {
        assert_eq!(hp, npc_hp_before - 5);
    } else {
        panic!("Expected NPC state");
    }
}
#[tokio::test]
async fn test_debuff_armor_second_case() {
    let debuff_card = Card::debuff_armor(1);
    let mut cards: Vec<Card> = vec![];
    cards.append(
        (0..6)
            .map(|_| debuff_card.clone())
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    cards.append(
        (0..6)
            .map(|_| Card::attack(5, DamageType::Normal))
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await; //monster
                                                                                              //starts with 2 armor
    let npc_hp_before = context.npc().await.get_hp();
    //add the debuff
    context.end_turn().await.unwrap();

    context
        .npc_turn(
            UseSpell(Spell::buff_armor(1)), //add some buff armor to npc
        )
        .await
        .unwrap();

    context.end_turn_npc().await.expect("Failed to end turn");

    let debuff_card = match context
        .find_card_recursive(|e| matches!(e, Effect::DebuffArmor { .. }))
        .await
    {
        Some(c) => c,
        None => {
            panic!("Expected debuff card");
        }
    };

    context.play_card_obj(debuff_card).await.unwrap();
    let attack_card = match context
        .find_card_recursive(|e| matches!(e, Effect::Damage { .. }))
        .await
    {
        Some(c) => c,
        None => {
            panic!("Expected attack card");
        }
    };

    context.play_card_obj(attack_card).await.unwrap();

    if let CombatantState::Npc { hp, .. } = context.get_encounter_state().await.unwrap().npc_state {
        assert_eq!(hp, npc_hp_before - 5 + 2);
    } else {
        panic!("Expected NPC state");
    }
}

#[tokio::test]
async fn test_silence_effect() {
    let cards: Vec<Card> = (0..12).map(|_| Card::silence(1)).collect();
    let context = CombatTestContext::new(create_test_hero!(cards), Monster::default()).await;
    context.play_card(0).await.unwrap();
    context.end_turn().await.unwrap();
    let encounter = context.get_encounter().await;
    assert_eq!(
        encounter.whos_turn(),
        CombatantIndex::Player,
        "Npc should lose a turn"
    );
    context.end_turn().await.unwrap();
    let encounter = context.get_encounter().await;
    assert_eq!(
        encounter.whos_turn(),
        CombatantIndex::Npc,
        "Silence should only last 1 round"
    );
}

#[tokio::test]
async fn test_dual_effect_mana_gain_attack_effect() {
    //attacks and gives mana
    let mut cards: Vec<Card> = (0..6)
        .map(|_| Card::attack(5, DamageType::Normal))
        .collect();
    cards.append(
        (0..6)
            .map(|_| Card::mana_gain_and_attack(2, 2))
            .collect::<Vec<Card>>()
            .as_mut(),
    );
    let npc = Monster::default().to_mut().armor(0);
    let context = CombatTestContext::new(create_test_hero!(cards), npc.clone()).await;
    let attack_card = context
        .find_card_recursive_by_type(|c| matches!(c.card_type, CardType::Attack))
        .await
        .unwrap();
    context.play_card_obj(attack_card).await.unwrap();
    let mana_card = context
        .find_card_recursive_by_type(|c| matches!(c.card_type, CardType::Utility))
        .await
        .unwrap();
    context.play_card_obj(mana_card).await.unwrap();

    if let CombatantState::Npc { hp, .. } = context.get_encounter_state().await.unwrap().npc_state {
        assert_eq!(
            npc.get_hp() - (5 + 2),
            hp,
            "Expected damage taken to be correct"
        );
    }

    if let CombatantState::Player { mana, .. } =
        context.get_encounter_state().await.unwrap().player_state
    {
        assert_eq!(3, mana);
    }
}

#[tokio::test]
async fn test_buff_damage_buff_armor_with_poison_combo() {
    // hero has buff on damage, npc has buff on armor and poison applied
    let mut cards: Vec<Card> = (0..6).map(|_| Card::buff_damage(2, Some(1))).collect();
    cards.append(
        (0..3)
            .map(|_| Card::poison(2, Some(1)))
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    cards.append(
        (0..3)
            .map(|_| Card::attack(5, DamageType::Normal))
            .collect::<Vec<Card>>()
            .as_mut(),
    );

    let context =
        CombatTestContext::new(create_test_hero!(cards), Monster::default().armor(0)).await;

    let buff_damage_card = context
        .find_card_recursive(|e| matches!(e, Effect::BuffDamage { .. }))
        .await
        .unwrap();

    context.play_card_obj(buff_damage_card).await.unwrap();
    context.end_turn().await.unwrap();

    let npc = context.npc().await;

    context
        .npc_turn(UseSpell(Spell::buff_armor(2)))
        .await
        .unwrap();

    context.end_turn_npc().await.expect("Failed to end turn");

    let poison_card = context
        .find_card_recursive(|e| matches!(e, Effect::Poison { .. }))
        .await
        .unwrap();
    context.play_card_obj(poison_card).await.unwrap();

    let attack_card = context
        .find_card_recursive(|e| matches!(e, Effect::Damage { .. }))
        .await
        .unwrap();
    context.play_card_obj(attack_card).await.unwrap();

    if let CombatantState::Npc { hp, .. } = context.get_encounter_state().await.unwrap().npc_state {
        assert_eq!(
            npc.get_hp() - 2 - (7 - 2),
            hp,
            "Expected damage taken to be correct"
        );
    }
}

#[tokio::test]
async fn test_zeal_accumulates() {}

#[tokio::test]
async fn test_cards_auto_shuffle_when_deck_empty() {}

#[tokio::test]
async fn test_combat_ends_after_kill() {}
