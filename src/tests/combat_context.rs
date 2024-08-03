// In your test_helpers.rs file

use std::sync::Arc;

use crate::events::combat::{CombatEncounter, CombatTurnMessage, EncounterState};
use crate::models::cards::{Card, Effect};
use crate::models::combatant::{Combatant, CombatantType};
use crate::models::hero_combatant::HeroCombatant;
use crate::models::npc::Monster;
use crate::prisma::{CardType, DamageType};
use crate::services::impls::combat_controller::{
    CombatCommand, CombatController, ControllerMessage,
};
use tokio::sync::{mpsc, oneshot};

use super::helpers::init_test_combat;

pub struct CombatTestContext {
    player_tx: mpsc::Sender<CombatCommand>,
    npc_tx: mpsc::Sender<CombatCommand>,
    controller_tx: mpsc::Sender<ControllerMessage>,
    encounter_id: String,
    hero_id: String,
    monster_id: String,
    pub combat_controller: Arc<CombatController>,
}

use DamageType::*;

impl CombatTestContext {
    pub async fn new(hero: HeroCombatant, monster: Monster) -> Self {
        let (player_tx, npc_tx, controller_tx, encounter_id, combat_controller) =
            init_test_combat(hero.clone(), monster.clone()).await;
        Self {
            player_tx,
            npc_tx,
            controller_tx,
            encounter_id,
            hero_id: hero.get_id(),
            monster_id: monster.get_id(),
            combat_controller,
        }
    }

    pub async fn get_encounter(&self) -> CombatEncounter {
        self.combat_controller
            .get_encounter(&self.encounter_id)
            .await
            .unwrap()
            .get()
            .to_owned()
    }

    pub async fn sync_encounter(
        &self,
        encounter: CombatEncounter,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.combat_controller.set_encounter(&encounter).await?;
        Ok(())
    }

    pub async fn get_hero_cards(&self) -> Vec<Card> {
        let hero = self.get_hero().await.unwrap();
        hero.get_hand().to_vec()
    }

    pub async fn play_card(&self, index: usize) -> Result<String, Box<dyn std::error::Error>> {
        let hero = self.get_hero().await?;
        let card = hero.get_hand()[index].clone();
        let id = card.id.clone();
        self.player_tx.send(CombatCommand::PlayCard(card)).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(id)
    }

    pub async fn play_card_obj(&self, card: Card) -> Result<(), Box<dyn std::error::Error>> {
        self.player_tx.send(CombatCommand::PlayCard(card)).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
    }

    pub async fn find_card_of_type(&self, effect_type: fn(&Effect) -> bool) -> Option<Card> {
        self.get_hero_cards()
            .await
            .into_iter()
            .find(|c| c.effects.iter().any(|e| effect_type(&e.effect)))
    }

    pub async fn find_card_recursive_by_type(
        &self,
        matcher: for<'a> fn(&'a Card) -> bool,
    ) -> Option<Card> {
        for _ in 0..10 {
            if let Some(card) = self.get_hero_cards().await.into_iter().find(|c| matcher(c)) {
                return Some(card);
            } else {
                self.get_encounter()
                    .await
                    .player_combatant
                    .as_hero()
                    .shuffle_deck();
            }
        }
        None
    }

    pub async fn find_card_recursive(
        &self,
        matcher: for<'a> fn(&'a Effect) -> bool,
    ) -> Option<Card> {
        for _ in 0..10 {
            if let Some(card) = self.find_card_of_type(matcher).await {
                return Some(card);
            } else {
                self.get_encounter()
                    .await
                    .player_combatant
                    .as_hero()
                    .shuffle_deck();
            }
        }
        None
    }

    pub async fn shuffle_cards(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut encounter = self.get_encounter().await;
        encounter.player_combatant.as_hero().shuffle_deck();
        self.sync_encounter(encounter).await?;
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

    pub async fn npc(&self) -> Monster {
        let (tx, rx) = oneshot::channel();
        self.controller_tx
            .send(ControllerMessage::GetCombatant {
                combatant_id: self.monster_id.clone(),
                encounter_id: self.encounter_id.clone(),
                tx,
            })
            .await
            .expect("Failed to get monster combatant");

        match rx.await.expect("Failed to get monster combatant").unwrap() {
            CombatantType::Monster(monster) => monster,
            _ => panic!("Expected monster combatant"),
        }
    }

    pub async fn npc_turn(&self, msg: CombatCommand) -> Result<(), Box<dyn std::error::Error>> {
        self.npc_tx.send(msg).await?;
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        Ok(())
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

pub fn create_attack_hero(amount: i32) -> HeroCombatant {
    let mut hero = HeroCombatant::default();

    let cards_in_deck: Vec<Card> = (0..12).map(|_| Card::attack(amount, Normal)).collect();
    hero.deck.cards_in_deck = cards_in_deck;
    hero
}

pub fn create_chaos_attack_hero() -> HeroCombatant {
    let mut hero = HeroCombatant::default();

    let cards_in_deck: Vec<Card> = (0..12).map(|_| Card::attack(5, Chaos)).collect();
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
    // pattern for a single expression (either card or vec<card>)
   ($item:expr) => {{
        $crate::tests::combat_context::create_test_hero_impl_wrapper($item)
    }};    // Pattern for a Vec<Card>
    //
    ($cards:expr) => {{
        let cards: Vec<aion_server::models::cards::Card> = $cards;
        $crate::tests::combat_context::create_test_hero_impl(cards)
    }};
    // Pattern for multiple cards separated by commas
    ($($card:expr),+ $(,)?) => {
        $crate::tests::combat_context::create_test_hero_impl(vec![$($card),+])
    };
}

pub fn create_test_hero_impl_wrapper<T>(item: T) -> HeroCombatant
where
    T: Into<Vec<Card>>,
{
    create_test_hero_impl(item.into())
}

impl From<Card> for Vec<Card> {
    fn from(card: Card) -> Self {
        vec![card]
    }
}

pub use create_test_hero;
