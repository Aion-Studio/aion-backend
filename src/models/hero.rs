use std::collections::HashMap;

use anyhow::Result;
use prisma_client_rust::chrono::{DateTime, Duration, FixedOffset, Utc};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tracing::log::error;

use crate::events::game::ActionDurations;
use crate::infra::Infra;
use crate::models::cards::Deck;
use crate::models::hero_combatant::HeroCombatant;
use crate::prisma::{stamina, Class, Resource as ResourcePrisma};
use crate::services::tasks::action_names::{ActionNames, TaskLootBox};
use crate::{
    events::game::ActionCompleted,
    prisma::{hero, hero_resource},
};

use super::region::RegionName;
use super::resources::{Relic, Resource};
use super::talent::{Spell, Talent};

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub id: Option<String>,
    pub hp: i32,
    pub name: String,
    pub class: Class,
    pub level: i32,
    pub strength: i32,
    pub armor: i32,
    pub intelligence: i32,
    pub dexterity: i32,
    pub explore: i32,
    pub crafting: i32,
    pub resources: HashMap<Resource, i32>,
    pub stamina: Stamina,
    pub decks: Option<Vec<Deck>>,
    pub spells: Vec<Spell>,
    pub relics: Vec<Relic>,
}

impl Class {
    pub fn get_rand() -> Class {
        let mut rng = thread_rng();
        match rng.gen_range(0..3) {
            0 => Class::Ranger,
            1 => Class::Fighter,
            _ => Class::Wizard,
        }
    }
}

impl Hero {
    pub fn new(hp: i32, strength: i32, dexterity: i32, class: Class) -> Self {
        let mut rng = thread_rng();
        Self {
            id: None,
            name: Self::generate_hero_name(),
            hp,
            class,
            level: 1,
            strength,
            armor: 1,
            dexterity,
            intelligence: rng.gen_range(0..5),
            resources: Resource::randomize_amounts(),
            crafting: 20,
            stamina: Stamina::new(),
            explore: 15,
            decks: None,
            spells: vec![],
            relics: vec![],
        }
    }

    pub fn default() -> Self {
        Self {
            id: Some(uuid::Uuid::new_v4().to_string()),
            name: Self::generate_hero_name(),
            hp: 100,
            class: Class::Ranger,
            level: 1,
            strength: rand::random::<i32>() % 10,
            armor: rand::random::<i32>() % 2,
            intelligence: rand::random::<i32>() % 3,
            dexterity: rand::random::<i32>() % 5,
            explore: 10,
            crafting: rand::random::<i32>() % 10,
            resources: Resource::randomize_amounts(),
            stamina: Stamina::new(),
            decks: None,
            spells: vec![],
            relics: vec![],
        }
    }

    pub fn active_deck(&self) -> Deck {
        let deck = match self
            .decks
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|deck| deck.active)
        {
            Some(deck) => deck.clone(),
            None => {
                //take first from decks vec
                let first_deck = self.decks.as_ref().unwrap().first().unwrap().clone();
                first_deck
            }
        };
        deck
    }

    pub fn to_combatant(&self) -> HeroCombatant {
        HeroCombatant::new(self.clone(), self.active_deck(), self.relics.clone())
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn regenerate_stamina(&mut self, res: &ActionCompleted) {
        let now = Utc::now();

        // Calculate seconds since last update only if last_stamina_regeneration_time is Some
        if let Some(last_regeneration_time) = self.stamina.last_regen_time {
            let seconds = now
                .signed_duration_since(last_regeneration_time)
                .num_seconds() as i32;

            let stamina_to_add = ((seconds as f64)
                * (Hero::calculate_stamina_regen_rate(self.level.clone(), self.intelligence.clone())
                    as f64
                    / 100.0))
                .round() as i32;

            // Add to self.stamina only if it is less than self.stamina_max
            if self.stamina.amount + stamina_to_add < self.stamina.capacity {
                self.stamina.amount += stamina_to_add;
            } else {
                self.stamina.amount = self.stamina.capacity;
            }
        } else {
            // This is the first time we're regenerating stamina, so no need to add anything yet
            // Just set the last regeneration time to now
        }

        // Update the last stamina regeneration time to now, regardless of whether we regenerated stamina
        self.stamina.last_regen_time = Some(now);
    }

    pub fn deduct_stamina(&mut self, stamina: i32) {
        self.stamina.amount -= stamina;
    }

    // adds the loot onto the hero struct
    pub fn equip_loot(&mut self, loot: TaskLootBox) {
        match loot {
            TaskLootBox::Region(result) => {
                let xp = result.xp;
                // find the resource enum type in the  self.resources and increase the amount by result.resources
                self.add_resources(result.resources);
            }
            TaskLootBox::Channel(result) => {
                let hero_id = result.hero_id.clone();
                let xp = result.xp;
                self.gain_stamina(result.stamina_gained);
                self.add_resources(result.resources);
            }
            TaskLootBox::Quest(result) => {
                self.add_resources(result.resources);
            }
            TaskLootBox::Raid(result) => {
                self.add_resources(result.resources);
            }
        }
    }

    fn add_resources(&mut self, resources: HashMap<Resource, i32>) {
        resources.iter().for_each(|(resource, amount)| {
            self.resources
                .entry(resource.clone())
                .and_modify(|r| *r += amount)
                .or_insert(*amount);
        });
    }

    pub async fn update_stats(&mut self, loot_box: &TaskLootBox) -> Result<()> {
        match loot_box {
            TaskLootBox::Region(result) => {
                let xp = result.xp;
            }
            TaskLootBox::Channel(result) => {
                let hero_id = result.hero_id.clone();
                let xp = result.xp;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn can_channel(&self, leyline_name: &str) -> bool {
        let leylines = Infra::repo().leylines_by_discovery(&self.get_id()).await;
        match leylines {
            Ok(leylines) => {
                return leylines.iter().any(|leyline| leyline.name == leyline_name);
            }
            Err(e) => {
                error!("Error getting leylines: {}", e);
                return false;
            }
        }
    }

    pub async fn get_current_region(&self) -> Result<RegionName> {
        let hero_regions = Infra::repo()
            .get_hero_regions(self.get_id().as_ref())
            .await?;
        let current_region = hero_regions
            .into_iter()
            .find(|hr| hr.current_location == true)
            .unwrap();
        Ok(current_region.region_name)
    }

    pub async fn timeout_durations(&self, action_name: &ActionNames) -> Duration {
        let timeout_duration = ActionDurations::timeouts(action_name);
        timeout_duration
    }
}

impl Hero {
    pub fn get_id(&self) -> String {
        self.id.clone().unwrap()
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        // Update other stats as per your game logic
    }

    pub fn gain_stamina(&mut self, stamina: i32) {
        // add stamina up to stamina_max
        if self.stamina.amount + stamina > self.stamina.capacity {
            self.stamina.amount = self.stamina.capacity;
            return;
        }
        self.stamina.amount += stamina;
    }

    // smooth curve to level 60

    fn adjusted_logarithmic_function(x: i32, attribute: i32) -> i32 {
        // Ensure attribute is within the valid range
        let attribute = attribute.clamp(1, 15) as f64;

        let x_transformed = (x as f64) / 60.0;
        let base_value = 1.5 + 7.5 * ((1.0 + 9.0 * x_transformed).ln() / 10.0_f64.ln());

        // Calculate the attribute multiplier
        let intelligence_boost = 1.0 + 0.35 * ((attribute - 1.0) / 14.0);

        // Apply the attribute multiplier
        let result = base_value * intelligence_boost;

        result.round() as i32
    }

    pub fn calculate_stamina_regen_rate(level: i32, intelligence: i32) -> i32 {
        Hero::adjusted_logarithmic_function(level, intelligence)
    }

    fn generate_hero_name() -> String {
        let syllables = [
            "Ar", "Al", "Bal", "Bel", "Bor", "Bra", "Ca", "Cru", "Da", "Dra", "El", "Fal", "Gor",
            "Gul", "Hel", "Il", "Ka", "Kru", "Lo", "Ma", "Mor", "Na", "No", "Or", "Ra", "Ro", "Sa",
            "Sha", "Ta", "Tha", "Ur", "Va", "Vor", "Za", "Zo",
        ];

        let titles = [
            "the Brave",
            "the Wise",
            "the Mighty",
            "the Dark",
            "the Lightbringer",
            "the Elder",
            "the Young",
            "the Swift",
            "the Stout",
            "the Fierce",
            "the Patient",
            "the Unyielding",
            "the Wanderer",
            "the Exiled",
            "the Slayer",
        ];

        let mut rng = thread_rng();
        let name_length = rng.gen_range(2..=3); // Decide if the name will have 2 or 3 syllables

        let mut name = String::new();
        for _ in 0..name_length {
            let syllable = syllables.choose(&mut rng).unwrap();
            name.push_str(syllable);
        }

        // Occasionally append a title
        if rng.gen_bool(0.3) {
            // 30% chance to append a title
            let title = titles.choose(&mut rng).unwrap();
            name = format!("{} {}", name, title);
        }

        name
    }
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl Range<i32> {
    pub fn roll(&self) -> i32 {
        thread_rng().gen_range(self.min..self.max)
    }
}

impl From<hero::Data> for Hero {
    fn from(data: hero::Data) -> Self {
        // Unwrapping the Option values and converting the data from each field
        // If the field is None, we provide a default value using the Default trait

        let resources = if let Some(resources) = &data.hero_resources {
            // Check if any resource is None
            if resources.iter().any(|r| r.resource.is_none()) {
                HashMap::new()
            } else {
                // Proceed with original logic, assuming all resources are Some
                HashMap::from_iter(resources.into_iter().map(|r| {
                    let resource_type = r.clone().into();
                    let amount = r.amount;
                    (resource_type, amount)
                }))
            }
        } else {
            HashMap::new()
        };

        Self {
            id: Some(data.id),
            name: data.name,
            class: data.class,
            hp: data.hp,
            level: data.level,
            strength: data.strength,
            armor: data.armor,
            intelligence: data.intelligence,
            dexterity: data.dexterity,
            explore: data.explore,
            crafting: data.crafting,
            spells: data.hero_spells.map_or(vec![], |spells| {
                spells
                    .into_iter()
                    .map(|spell| Spell::from(spell))
                    .collect::<Vec<Spell>>()
            }),
            stamina: data
                .stamina
                .map(|stamina| Stamina::from(stamina))
                .unwrap_or_default(),
            resources,
            relics: data.relics.map_or(vec![], |relics| {
                relics
                    .into_iter()
                    .map(|relic| Relic::from(relic))
                    .collect::<Vec<Relic>>()
            }),

            decks: None, // we fill in the deck manually
        }
    }
}

pub fn convert_to_utc(dt: Option<DateTime<FixedOffset>>) -> Option<DateTime<Utc>> {
    dt.map(|datetime| datetime.with_timezone(&Utc))
}

#[allow(deprecated)]
pub fn convert_to_fixed_offset(dt: Option<DateTime<Utc>>) -> Option<DateTime<FixedOffset>> {
    dt.map(|datetime| datetime.with_timezone(&FixedOffset::east(0)))
}

impl From<hero_resource::Data> for Resource {
    fn from(data: hero_resource::Data) -> Self {
        if let Some(resource_data) = data.resource {
            match resource_data.r#type {
                ResourcePrisma::Aion => Resource::Aion,
                ResourcePrisma::Gem => Resource::Gem,
                ResourcePrisma::Flux => Resource::Flux,
            }
        } else {
            panic!("Invalid resource type!")
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct Stamina {
    pub capacity: i32,
    pub amount: i32,
    pub last_regen_time: Option<DateTime<Utc>>,
}

impl Stamina {
    pub fn new() -> Self {
        Self {
            capacity: 100,
            amount: 100,
            last_regen_time: None,
        }
    }
}

impl From<Option<Box<stamina::Data>>> for Stamina {
    fn from(data: Option<Box<stamina::Data>>) -> Self {
        if let Some(data) = data {
            Self {
                capacity: data.capacity,
                amount: data.amount,
                last_regen_time: convert_to_utc(data.last_regen_time),
            }
        } else {
            Stamina::new()
        }
    }
}
