use std::collections::HashMap;

use prisma_client_rust::chrono::{self, Duration};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tracing::log::error;

use super::region::RegionName;
use super::resources::Resource;
use crate::events::game::{ActionDurations, ActionNames};
use crate::infra::Infra;
use crate::{
    events::game::{ActionCompleted, TaskLootBox},
    prisma::{
        attributes, base_stats, follower, hero, hero_resource, inventory, item, retinue_slot,
        ResourceType,
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
    pub aion_capacity: i32,
    pub aion_collected: i32,
    pub stamina: i32,
    pub stamina_max: i32,
    pub stamina_regen_rate: i32,
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
            aion_collected,
            stamina: 100,
            stamina_max: 100,
            stamina_regen_rate: 1,
        }
    }

    pub fn regenerate_stamina(&mut self, res: &ActionCompleted) {
        // set the self.stamina to number of seconds since last regionactionresult.created time and now
        // multiplied by self.stamina_regen_rate
        let now = chrono::Utc::now();
        let seconds = now.signed_duration_since(res.updated_at).num_seconds() as i32;

        let stamina = seconds * self.stamina_regen_rate;
        // add to self.stamina only if it is less than self.stamina_max
        if self.stamina + stamina < self.stamina_max {
            self.stamina += stamina;
        } else {
            self.stamina = self.stamina_max;
        }
    }

    pub fn deduct_stamina(&mut self, stamina: i32) {
        self.stamina -= stamina;
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
        }
    }

    fn add_resources(&mut self, resources: HashMap<Resource, i32>) {
        resources.iter().for_each(|resource| {
            self.resources
                .entry(resource.0.clone())
                .and_modify(|r| *r += resource.1);
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
        // let hero = Infra::repo()
        //     .get_hero(action.hero_id.clone())
        //     .await
        //     .unwrap();
        let timeout_duration = ActionDurations::timeouts(action_name);
        timeout_duration
    }
    // Add other methods as per your game logic
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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BaseStats {
    pub id: Option<String>,
    pub level: i32,
    pub xp: i32,
    pub damage: Range<i32>,
    pub hit_points: i32,
    pub mana: i32,
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
        let resources = match data.resources {
            Some(resources) => HashMap::from_iter(resources.into_iter().map(|r| {
                let resource_type = r.clone().into();
                let amount = r.amount;
                (resource_type, amount)
            })),
            None => HashMap::new(),
        };

        Self {
            id: Some(data.id),
            name: data.name,
            base_stats,
            attributes,
            inventory: Some(inventory),
            retinue_slots,
            aion_capacity: data.aion_capacity,
            aion_collected: data.aion_collected,
            stamina: data.stamina,
            stamina_max: data.stamina_max,
            stamina_regen_rate: data.stamina_regen_rate,
            resources,
        }
    }
}

impl From<base_stats::Data> for BaseStats {
    fn from(data: base_stats::Data) -> Self {
        Self {
            id: Some(data.id),
            level: data.level,
            xp: data.xp,
            damage: Range {
                min: data.damage_min,
                max: data.damage_max,
            },
            hit_points: data.hit_points,
            mana: data.mana,
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
        match data.resource {
            ResourceType::Aion => Resource::Aion,
            ResourceType::Valor => Resource::Valor,
            ResourceType::NexusShard => Resource::NexusShard,
            ResourceType::Oak => Resource::Oak,
            ResourceType::IronOre => Resource::IronOre,
            ResourceType::Copper => Resource::Copper,
            ResourceType::Silk => Resource::Silk,
        }
    }
}
