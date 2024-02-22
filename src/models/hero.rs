use std::any::Any;
use std::collections::HashMap;

use prisma_client_rust::chrono::{self, DateTime, Duration, FixedOffset, Utc};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing::log::error;

use super::combatant::Combatant;
use super::region::RegionName;
use super::resources::Resource;
use super::talent::{Effect, Talent};
use crate::events::game::ActionDurations;
use crate::infra::Infra;
use crate::prisma::ResourceEnum;
use crate::services::tasks::action_names::{ActionNames, TaskLootBox};
use crate::{
    events::game::ActionCompleted,
    prisma::{
        attributes, base_stats, follower, hero, hero_resource, inventory, item, retinue_slot,
    },
};
use anyhow::Result;

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub id: Option<String>,
    pub name: String,
    pub base_stats: BaseStats,
    pub attributes: Attributes,
    pub inventory: Option<Inventory>,
    pub retinue_slots: Vec<RetinueSlot>,
    pub resources: HashMap<Resource, i32>,
    pub aion_capacity: i32, // NOTO: This is not used anywhere at the moment
    pub stamina: i32,
    pub stamina_max: i32,
    pub stamina_regen_rate: i32,
    pub last_stamina_regeneration_time: Option<DateTime<Utc>>, // Add this
    pub talents: Vec<Talent>,
}

// methods to only update the model struct based on some calculation
impl Hero {
    pub fn new(
        base_stats: BaseStats,
        attributes: Attributes,
        aion_capacity: i32,
        aion_collected: i32,
    ) -> Self {
        Self {
            id: None,
            name: Self::generate_hero_name(),
            base_stats,
            inventory: None,
            attributes,
            retinue_slots: vec![],
            aion_capacity,
            resources: Resource::randomize_amounts(),
            stamina: 100,
            stamina_max: 100,
            stamina_regen_rate: 1,
            last_stamina_regeneration_time: None,
            talents: vec![],
        }
    }

    pub fn level(&self) -> i32 {
        self.base_stats.level
    }

    pub fn regenerate_stamina(&mut self, res: &ActionCompleted) {
        let now = Utc::now();

        // Calculate seconds since last update only if last_stamina_regeneration_time is Some
        if let Some(last_regeneration_time) = self.last_stamina_regeneration_time {
            let seconds = now
                .signed_duration_since(last_regeneration_time)
                .num_seconds() as i32;

            info!(
                "(self.stamina_regen_rate as f64 / 10.0)).round() {:?}",
                (seconds as f64 * self.stamina_regen_rate as f64 / 100.0).round()
            );
            let stamina_to_add =
                ((seconds as f64) * (self.stamina_regen_rate as f64 / 100.0)).round() as i32;
            info!("Stamina to be added: {}", stamina_to_add);

            // Add to self.stamina only if it is less than self.stamina_max
            if self.stamina + stamina_to_add < self.stamina_max {
                self.stamina += stamina_to_add;
            } else {
                self.stamina = self.stamina_max;
            }
        } else {
            // This is the first time we're regenerating stamina, so no need to add anything yet
            // Just set the last regeneration time to now
            info!("Setting initial stamina regeneration time");
        }

        // Update the last stamina regeneration time to now, regardless of whether we regenerated stamina
        self.last_stamina_regeneration_time = Some(now);
    }

    pub fn deduct_stamina(&mut self, stamina: i32) {
        self.stamina -= stamina;
    }

    pub fn deduct_shards(&mut self, cost: &i32) {
        self.resources
            .entry(Resource::StormShard)
            .and_modify(|r| *r -= cost);
    }

    // adds the loot onto the hero struct
    pub fn equip_loot(&mut self, loot: TaskLootBox) {
        match loot {
            TaskLootBox::Region(result) => {
                let xp = result.xp;
                self.gain_experience(xp);
                // find the resource enum type in the  self.resources and increase the amount by result.resources
                self.add_resources(result.resources);
            }
            TaskLootBox::Channel(result) => {
                let hero_id = result.hero_id.clone();
                let xp = result.xp;
                self.gain_experience(xp);
                self.gain_stamina(result.stamina_gained);
                self.add_resources(result.resources);
            }
            TaskLootBox::Quest(result) => {
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
                self.gain_experience(xp);
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

    pub fn level_calculator(xp: i32) -> i32 {
        let levels_1_10: [i32; 10] = [0, 200, 700, 1600, 3000, 5000, 7700, 10500, 13800, 17600];

        let levels_10_20_increase = 1500;
        let mut cumulative_xp = levels_1_10[9];
        let mut levels_10_20: [i32; 10] = [0; 10];
        for i in 0..10 {
            cumulative_xp += levels_10_20_increase;
            levels_10_20[i] = cumulative_xp;
        }

        let levels_20_30_increase = 2500;
        let mut levels_20_30: [i32; 10] = [0; 10];
        for i in 0..10 {
            cumulative_xp += levels_20_30_increase;
            levels_20_30[i] = cumulative_xp;
        }

        let levels_30_40_increase = 5000;
        let mut levels_30_40: [i32; 10] = [0; 10];
        for i in 0..10 {
            cumulative_xp += levels_30_40_increase;
            levels_30_40[i] = cumulative_xp;
        }

        let levels_40_50_increase = 10000;
        let mut levels_40_50: [i32; 10] = [0; 10];
        for i in 0..10 {
            cumulative_xp += levels_40_50_increase;
            levels_40_50[i] = cumulative_xp;
        }

        let levels_50_60: [i32; 10] = [
            cumulative_xp + 250000,
            cumulative_xp + 600000,
            cumulative_xp + 1050000,
            cumulative_xp + 1600000,
            cumulative_xp + 2250000,
            cumulative_xp + 3000000,
            cumulative_xp + 3850000,
            cumulative_xp + 4800000,
            cumulative_xp + 5850000,
            cumulative_xp + 7050000,
        ];

        let level_thresholds: [i32; 60] = [
            levels_1_10,
            levels_10_20,
            levels_20_30,
            levels_30_40,
            levels_40_50,
            levels_50_60,
        ]
        .concat()
        .try_into()
        .unwrap();

        for (level, &threshold) in level_thresholds.iter().enumerate() {
            if xp < threshold {
                return level as i32;
            }
        }

        60 // If XP is beyond the last threshold, return 60}
    }
}

impl Hero {
    pub fn get_id(&self) -> String {
        self.id.clone().unwrap()
    }

    pub fn level_up(&mut self) {
        self.base_stats.level += 1;
        // Update other stats as per your game logic
    }

    pub fn gain_experience(&mut self, xp: i32) {
        self.base_stats.xp += xp;
        // Check for level up
    }

    pub fn gain_stamina(&mut self, stamina: i32) {
        // add stamina up to stamina_max
        if self.stamina + stamina > self.stamina_max {
            self.stamina = self.stamina_max;
            return;
        }
        self.stamina += stamina;
    }

    pub fn equip(&mut self, item: Item) {
        if let Some(inv) = &mut self.inventory {
            inv.active.push(item);
        }
    }

    pub fn equip_backpack(&mut self, item: Item) {
        if let Some(inv) = &mut self.inventory {
            inv.backpack.push(item);
        }
    }

    pub fn assign_follower(&mut self, slot: RetinueSlot) {
        self.retinue_slots.push(slot);
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

impl Combatant for Hero {
    fn get_id(&self) -> String {
        self.id.clone().unwrap()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_hp(&self) -> i32 {
        self.base_stats.hit_points
    }

    fn get_damage(&self) -> i32 {
        self.base_stats.damage.roll()
    }

    fn get_talents(&self) -> &Vec<Talent> {
        &self.talents
    }

    fn get_armor(&self) -> i32 {
        self.base_stats.armor
    }
    fn get_level(&self) -> i32 {
        self.base_stats.level
    }
    fn attack(&self, other: &mut dyn Combatant) {
        let damage = self.get_damage();
        other.take_damage(damage);
    }
    fn take_damage(&mut self, damage: i32) {
        self.base_stats.hit_points -= damage;
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseStats {
    pub id: Option<String>,
    pub level: i32,
    pub xp: i32,
    pub damage: Range<i32>,
    pub hit_points: i32,
    pub armor: i32,
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Attributes {
    pub id: Option<String>,
    pub resilience: i32,
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub exploration: i32,
    pub crafting: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeModifier {
    attribute: Attributes,
    // which attribute this modifier affects
    change: i32, // positive for increase, negative for decrease
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Inventory {
    pub hero_id: String,
    pub active: Vec<Item>,
    pub backpack: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum RetinueSlot {
    Mage(Follower),
    Warrior(Follower),
    Priest(Follower),
    Ranger(Follower),
    Alchemist(Follower),
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Follower {
    pub name: String,
    pub level: i32,
    pub bonus_attributes: Attributes,
    pub talents: Vec<Talent>,
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub weight: i32,
    pub value: i32,
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
        let base_stats = match data.base_stats {
            Some(bs) => (*bs).into(),     // Convert from base_stats::Data to BaseStats
            None => BaseStats::default(), // Provide a default value
        };

        let attributes = match data.attributes {
            Some(attr) => (*attr).into(), // Convert from attributes::Data to Attributes
            None => Attributes::default(), // Provide a default value
        };

        let inventory = match data.inventory {
            Some(inv) => (*inv).into(),   // Convert from inventory::Data to Inventory
            None => Inventory::default(), // Provide a default value
        };

        let retinue_slots = match data.retinue_slots {
            Some(rslots) => rslots.into_iter().map(RetinueSlot::from).collect(),
            None => vec![],
        };

        let resources = if let Some(resources) = &data.resources {
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
            base_stats,
            attributes,
            inventory: Some(inventory),
            retinue_slots,
            aion_capacity: data.aion_capacity,
            stamina: data.stamina,
            stamina_max: data.stamina_max,
            stamina_regen_rate: data.stamina_regen_rate,
            resources,
            last_stamina_regeneration_time: convert_to_utc(data.last_stamina_regeneration_time),
            talents: match data.hero_talents {
                Some(talents) => talents.into_iter().map(Talent::from).collect(),
                None => vec![],
            },
        }
    }
}

pub fn convert_to_utc(dt: Option<DateTime<FixedOffset>>) -> Option<DateTime<Utc>> {
    dt.map(|datetime| datetime.with_timezone(&Utc))
}

pub fn convert_to_fixed_offset(dt: Option<DateTime<Utc>>) -> Option<DateTime<FixedOffset>> {
    dt.map(|datetime| datetime.with_timezone(&FixedOffset::east(0)))
}

impl From<base_stats::Data> for BaseStats {
    fn from(data: base_stats::Data) -> Self {
        Self {
            id: Some(data.id),
            level: Hero::level_calculator(data.xp),
            xp: data.xp,
            damage: Range {
                min: data.damage_min,
                max: data.damage_max,
            },
            hit_points: data.hit_points,
            armor: data.armor,
        }
    }
}

impl From<attributes::Data> for Attributes {
    fn from(data: attributes::Data) -> Self {
        Self {
            id: Some(data.id),
            resilience: data.resilience,
            strength: data.strength,
            agility: data.agility,
            intelligence: data.intelligence,
            exploration: data.exploration,
            crafting: data.crafting,
        }
    }
}

impl From<item::Data> for Item {
    fn from(data: item::Data) -> Self {
        Item {
            id: data.id,
            name: data.name,
            weight: data.weight,
            value: data.value,
        }
    }
}

impl From<inventory::Data> for Inventory {
    fn from(data: inventory::Data) -> Self {
        let active = data
            .active
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(Item::from)
            .collect();

        let backpack = data
            .backpack
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(Item::from)
            .collect();
        Inventory {
            hero_id: data.id,
            active,
            backpack,
        }
    }
}

impl From<follower::Data> for Follower {
    fn from(data: follower::Data) -> Self {
        let attributes = match data.attributes {
            Some(attr) => (*attr).into(), // Convert from prisma::attributes::Data to Attributes
            None => Attributes::default(), // Provide a default value
        };

        Self {
            name: data.name,
            level: data.level,
            bonus_attributes: attributes,
            talents: match data.follower_talents {
                Some(talents) => talents.into_iter().map(Talent::from).collect(),
                None => vec![],
            },
        }
    }
}

impl From<retinue_slot::Data> for RetinueSlot {
    fn from(data: retinue_slot::Data) -> Self {
        let follower = data
            .follower
            .and_then(|f| f) // This line is used to flatten Option<Option<T>> to Option<T>
            .map(|f| (*f).into()) // Convert prisma::follower::Data to Follower
            .unwrap_or_default(); // Provide a default Follower if None

        match data.slot_type.as_str() {
            "Mage" => RetinueSlot::Mage(follower),
            "Warrior" => RetinueSlot::Warrior(follower),
            "Priest" => RetinueSlot::Priest(follower),
            "Ranger" => RetinueSlot::Ranger(follower),
            "Alchemist" => RetinueSlot::Alchemist(follower),
            _ => panic!("Invalid slot type!"), // Handle invalid slot_type appropriately
        }
    }
}

impl From<hero_resource::Data> for Resource {
    fn from(data: hero_resource::Data) -> Self {
        if let Some(resource_data) = data.resource {
            match resource_data.r#type {
                ResourceEnum::Aion => Resource::Aion,
                ResourceEnum::Valor => Resource::Valor,
                ResourceEnum::NexusOrb => Resource::NexusOrb,
                ResourceEnum::StormShard => Resource::StormShard,
            }
        } else {
            panic!("Invalid resource type!")
        }
    }
}
