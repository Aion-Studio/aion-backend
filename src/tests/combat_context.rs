// In your test_helpers.rs file

use crate::events::combat::{CombatTurnMessage, EncounterState};
use crate::models::cards::Card;
use crate::models::combatant::{Combatant, CombatantType};
use crate::models::hero_combatant::HeroCombatant;
use crate::models::npc::Monster;
use crate::services::impls::combat_service::{CombatCommand, ControllerMessage};
use tokio::sync::{mpsc, oneshot};

use super::helpers::init_test_combat;

pub struct CombatTestContext {
    player_tx: mpsc::Sender<CombatCommand>,
    npc_tx: mpsc::Sender<CombatCommand>,
    controller_tx: mpsc::Sender<ControllerMessage>,
    encounter_id: String,
    hero_id: String,
    monster_id: String,
}

impl CombatTestContext {
    pub async fn new(hero: HeroCombatant, monster: Monster) -> Self {
        let (player_tx, npc_tx, controller_tx, encounter_id) =
            init_test_combat(hero.clone(), monster.clone()).await;
        Self {
            player_tx,
            npc_tx,
            controller_tx,
            encounter_id,
            hero_id: hero.get_id(),
            monster_id: monster.get_id(),
        }
    }

    pub async fn play_card(&self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let hero = self.get_hero().await?;
        let card = hero.get_hand()[index].clone();
        self.player_tx.send(CombatCommand::PlayCard(card)).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    pub async fn end_turn_npc(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.npc_tx.send(CombatCommand::EndTurn).await {
            Ok(_) => {}
            Err(e) => panic!("Failed to send end turn command to NPC {:?}", e),
        };
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    pub async fn end_turn(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.player_tx.send(CombatCommand::EndTurn).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    pub async fn get_hero(&self) -> Result<HeroCombatant, Box<dyn std::error::Error>> {
        let (tx, rx) = oneshot::channel();
        self.controller_tx
            .send(ControllerMessage::GetCombatant {
                combatant_id: self.hero_id.clone(),
                encounter_id: self.encounter_id.clone(),
                tx,
            })
            .await?;

        match rx.await?.unwrap() {
            CombatantType::Hero(hero) => Ok(hero),
            _ => panic!("Expected hero combatant"),
        }
    }

    pub async fn get_monster(&self) -> Result<Monster, Box<dyn std::error::Error>> {
        let (tx, rx) = oneshot::channel();
        self.controller_tx
            .send(ControllerMessage::GetCombatant {
                combatant_id: self.monster_id.clone(),
                encounter_id: self.encounter_id.clone(),
                tx,
            })
            .await?;

        match rx.await?.unwrap() {
            CombatantType::Monster(monster) => Ok(monster),
            _ => panic!("Expected monster combatant"),
        }
    }

    pub async fn get_encounter_state(&self) -> Result<EncounterState, Box<dyn std::error::Error>> {
        let (tx, rx) = oneshot::channel();
        self.controller_tx
            .send(ControllerMessage::RequestState {
                combatant_id: self.hero_id.clone(),
                tx,
            })
            .await?;

        match rx.await? {
            (Some(CombatTurnMessage::EncounterData(encounter_state)), _) => Ok(encounter_state),
            _ => panic!("Failed to get encounter state"),
        }
    }
}

pub fn create_poison_hero() -> HeroCombatant {
    let mut hero = HeroCombatant::default();
    let cards_in_deck: Vec<Card> = (0..12).map(|_| Card::poison(2, Some(2))).collect();
    hero.deck.cards_in_deck = cards_in_deck;
    hero
}

pub fn create_attack_hero() -> HeroCombatant {
    let mut hero = HeroCombatant::default();

    let cards_in_deck: Vec<Card> = (0..12).map(|_| Card::attack(2)).collect();
    hero.deck.cards_in_deck = cards_in_deck;
    hero
}

pub fn create_test_hero_impl(cards: Vec<Card>) -> HeroCombatant {
    let mut hero = HeroCombatant::default();

    let cards_in_deck = if cards.len() == 1 {
        vec![cards[0].clone(); 12]
    } else {
        cards
    };

    hero.deck.cards_in_deck = cards_in_deck;
    hero
}

#[macro_export]
macro_rules! create_test_hero {
    ($($card:expr),+ $(,)?) => {
        $crate::tests::combat_context::create_test_hero_impl(vec![$($card),+])
    };
}

pub use create_test_hero;
