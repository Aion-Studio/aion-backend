use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use actix::Message;
use serde::{Deserialize, Serialize};
use tracing::log::info;

use CombatantIndex::*;

use crate::models::cards::{Card, CardEffect};
use crate::models::combatant::CombatantType;
use crate::models::hero_combatant::HeroCombatant;
use crate::models::resources::Relic;
use crate::models::talent::Spell;
use crate::prisma::{EffectType, TargetType};
use crate::{
    models::{combatant::Combatant, npc::Monster},
    services::impls::combat_service::CombatCommand,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CombatantIndex {
    Player,
    Npc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatEncounter {
    id: String,
    pub player_combatant: CombatantType,
    pub npc_combatant: CombatantType,
    pub round: i32,
    pub action_id: Option<String>,
    active_effects: HashMap<String, Vec<CardEffect>>,
    current_turn: CombatantIndex,
    pub started: bool,
    initial_hps: (i32, i32), // comb1 and comb2
}

impl CombatEncounter {
    pub fn new(hero: HeroCombatant, npc: Monster) -> Self {
        let hero_hp = hero.get_hp();
        let monster_hp = npc.get_hp();
        let player_combatant = CombatantType::Hero(hero);
        let npc_combatant = CombatantType::Monster(npc);

        CombatEncounter {
            id: uuid::Uuid::new_v4().to_string(),
            player_combatant,
            npc_combatant, // Box the generic monster
            active_effects: HashMap::new(),
            current_turn: CombatantIndex::Player,
            started: false,
            initial_hps: (hero_hp, monster_hp),
            action_id: None,
            round: 1,
        }
    }

    pub fn get_player_combatant(&mut self) -> &mut CombatantType {
        &mut self.player_combatant
    }
    pub fn set_action_id(&mut self, action_id: String) {
        self.action_id = Some(action_id);
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn add_active_effect(&mut self, target_id: String, effect: CardEffect) {
        self.active_effects
            .entry(target_id)
            .or_insert_with(Vec::new)
            .push(effect);
    }

    pub fn get_combatant_ids(&self) -> Vec<String> {
        vec![
            self.player_combatant.as_combatant().get_id(),
            self.npc_combatant.as_combatant().get_id(),
        ]
    }

    pub fn has_combatant(&self, combatant_id: &str) -> bool {
        self.player_combatant.as_combatant().get_id() == combatant_id
            || self.npc_combatant.as_combatant().get_id() == combatant_id
    }

    pub fn whos_turn(&self) -> CombatantIndex {
        self.current_turn.clone()
    }
    pub fn get_combatant(
        &mut self,
        index: &CombatantIndex,
        is_opponent: Option<bool>,
    ) -> &mut CombatantType {
        match index {
            Player => {
                if is_opponent.unwrap_or(false) {
                    &mut self.npc_combatant
                } else {
                    &mut self.player_combatant
                }
            }
            Npc => {
                if is_opponent.unwrap_or(false) {
                    &mut self.player_combatant
                } else {
                    &mut self.npc_combatant
                }
            }
        }
    }

    pub fn get_combatant_by_id(&mut self, combatant_id: &str) -> Option<&mut CombatantType> {
        let player_combatant_id = self.player_combatant.as_combatant().get_id();
        if combatant_id == player_combatant_id {
            Some(&mut self.player_combatant)
        } else {
            Some(&mut self.npc_combatant)
        }
    }

    pub fn get_opponent(&mut self, combatant_id: &str) -> &mut CombatantType {
        if combatant_id == self.player_combatant.as_combatant().get_id() {
            &mut self.npc_combatant
        } else {
            &mut self.player_combatant
        }
    }

    pub fn get_combatant_idx(&self, combatant_id: &str) -> Option<CombatantIndex> {
        if combatant_id == self.player_combatant.as_combatant().get_id() {
            Some(Player)
        } else if combatant_id == self.npc_combatant.as_combatant().get_id() {
            Some(Npc)
        } else {
            None
        }
    }

    // shuffles deck and draws 5 cards
    pub fn initialize(&mut self) {
        if self.started {
            info!("Combat already started");
            return;
        }
        let combatant_type = self.get_combatant(&Player, None);
        let combatant = combatant_type.as_combatant_mut();

        if combatant.get_hand().is_empty() {
            combatant.shuffle_deck();
            combatant.draw_cards();
        }
        self.started = true;
    }

    fn handle_play_card(
        &mut self,
        card: Card,
        caster_id: &str,
    ) -> Result<CombatTurnMessage, CombatError> {
        let mut effects_to_apply = Vec::new();
        {
            let (caster, opponent) = if caster_id == self.player_combatant.as_combatant().get_id() {
                (&mut self.player_combatant, &mut self.npc_combatant)
            } else if caster_id == self.npc_combatant.as_combatant().get_id() {
                (&mut self.npc_combatant, &mut self.player_combatant)
            } else {
                return Err(CombatError::CombatantNotFound);
            };

            if caster.as_combatant().get_mana() < card.cost {
                return Err(CombatError::ManaError);
            }

            for effect in &card.effects {
                let target_id = match effect.target_type {
                    TargetType::Itself => caster.as_combatant().get_id(),
                    TargetType::Opponent => opponent.as_combatant().get_id(),
                };

                effects_to_apply.push((target_id, effect.clone()));
            }
        }

        self.apply_effects(effects_to_apply)?;

        let caster_combatant = self.get_combatant_by_id(caster_id).unwrap();

        // Deduct mana and zeal
        caster_combatant.as_combatant_mut().spend_mana(card.cost);

        // play and discard
        caster_combatant
            .as_combatant_mut()
            .play_card(&card)
            .unwrap();

        Ok(CombatTurnMessage::CommandPlayed(CombatCommand::PlayCard(
            card,
        )))
    }

    fn apply_effects(&mut self, effects: Vec<(String, CardEffect)>) -> Result<(), CombatError> {
        for (target_id, effect) in effects {
            let target = if target_id == self.player_combatant.as_combatant().get_id() {
                &mut self.player_combatant
            } else {
                &mut self.npc_combatant
            }
            .as_combatant_mut();

            let effect_clone = effect.clone();
            match effect.effect {
                EffectType::Damage => {
                    let damage = effect.value;
                    target.take_damage(damage);
                }
                EffectType::Heal => {
                    target.heal(effect.value);
                }
                EffectType::Poison => {
                    self.add_active_effect(target_id, effect_clone);
                }
                EffectType::BuffStat | EffectType::DebuffStat => {}
                EffectType::Draw => {}
                EffectType::Initiative => {}
                EffectType::Armor => {}
                EffectType::DebuffDamage => {}
                EffectType::Silence => {}
                EffectType::ManaGain => {}
                EffectType::BuffDamage => {}
            }
        }
        Ok(())
    }

    pub fn process_combat_turn(
        &mut self,
        cmd: CombatCommand,
        combatant_id: &str, // the ID of the combatant making the move
    ) -> Result<CombatTurnMessage, CombatError> {
        use CombatCommand::*;
        let idx = self.get_combatant_idx(combatant_id).unwrap();
        let is_valid_turn = self.current_turn == idx;
        if !is_valid_turn {
            return Err(CombatError::OutOfTurnAction);
        }
        let result = match cmd.clone() {
            PlayCard(card) => match self.handle_play_card(card, combatant_id) {
                Ok(msg) => Ok(msg),
                Err(e) => Err(e),
            },
            EndTurn => {
                self.current_turn = match idx {
                    Player => CombatantIndex::Npc,
                    Npc => Player,
                };

                // increment round draws cards for combatant 1, else statement draws for combatant 2
                if self.current_turn == Player {
                    self.increment_round();
                    let player = self.player_combatant.as_combatant_mut();
                    player.draw_cards();
                    player.add_mana();
                }

                self.apply_round_start_effects();
                Ok(CombatTurnMessage::CommandPlayed(cmd.clone()))
            }
            _ => {
                todo!()
            }
        };

        result
    }

    fn apply_round_start_effects(&mut self) {
        let target = match self.current_turn {
            Player => &mut self.player_combatant,
            Npc => &mut self.npc_combatant,
        }
        .as_combatant_mut();

        if let Some(effects) = self.active_effects.get_mut(&target.get_id()) {
            let mut to_remove = vec![];
            for effect in effects {
                match effect.effect {
                    EffectType::Poison => {
                        target.take_damage(effect.value);
                        // decrement duration of effect, if resulting is 0 , add to remove ids vec
                        // and then remove after effects loop
                        effect.duration = effect.duration.map(|d| d - 1);
                        if effect.duration == Some(0) {
                            to_remove.push(effect.id.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        self.active_effects
            .entry(target.get_id())
            .and_modify(|effects| {
                effects.retain(|effect| effect.duration != Some(0));
            });
    }

    fn check_skip_turn_effects(&mut self) -> Option<()> {
        unimplemented!()
    }

    fn increment_round(&mut self) {
        self.round += 1;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CombatTurnMessage {
    CommandPlayed(CombatCommand),
    // Command played , result of the defender
    PlayerTurn(CombatantIndex),
    PlayerMissesTurn,
    Winner(CombatantIndex),
    PlayerState(CombatantState),
    EncounterData(EncounterState), // Requested state
    Temp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CombatantState {
    #[serde(rename_all = "camelCase")]
    Player {
        max_hp: i32,
        hp: i32,
        mana: i32,
        armor: i32,
        zeal: i32,
        strength: i32,
        intelligence: i32,
        dexterity: i32,
        spells: Vec<Spell>,
        relics: Vec<Relic>,
        drawn_cards: Vec<Card>,
        cards_in_discard: Vec<Card>,
    },
    #[serde(rename_all = "camelCase")]
    Npc {
        max_hp: i32,
        hp: i32,
        spells: Vec<Spell>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncounterState {
    pub turn: CombatantIndex,
    pub round: i32,
    pub player_state: CombatantState,
    pub npc_state: CombatantState,
}

impl Clone for CombatEncounter {
    fn clone(&self) -> Self {
        CombatEncounter {
            id: self.id.clone(),
            player_combatant: self.player_combatant.clone(),
            npc_combatant: self.npc_combatant.clone(),
            active_effects: self.active_effects.clone(),
            current_turn: self.current_turn.clone(),
            started: self.started,
            initial_hps: self.initial_hps,
            action_id: self.action_id.clone(),
            round: self.round,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum CombatantData {
    Monster(Monster),
    Hero(HeroCombatant),
}

impl Message for CombatTurnMessage {
    type Result = ();
}

#[derive(Debug)]
pub enum CombatError {
    OutOfTurnAction,
    ManaError,
    JustPlayedCardError,
    CardNotInHand, // ... (Other error variants as needed) ...
    CardNotFound,
    CombatantNotFound,
}

impl Display for CombatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatError::OutOfTurnAction => write!(f, "Out of turn action"),
            CombatError::ManaError => write!(f, "Not enough mana"),
            CombatError::CardNotInHand => write!(f, "Card not in hand"),
            CombatError::CardNotFound => write!(f, "Card not found"),
            CombatError::JustPlayedCardError => write!(f, "Just played card"),
            CombatError::CombatantNotFound => write!(f, "Combatant not found"),
        }
    }
}
