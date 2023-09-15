use prisma_client_rust::chrono;
use serde::{Deserialize, Serialize};

use crate::events::game::{RegionActionResult, TaskLootBox};
use anyhow::Result;
use crate::infra::Infra;
use super::resources::Resource;

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hero {
    pub id: Option<String>,
    pub base_stats: BaseStats,
    pub attributes: Attributes,
    pub inventory: Option<Inventory>,
    pub retinue_slots: Vec<RetinueSlot>,
    pub resources: Vec<Resource>,
    pub aion_capacity: i32,
    pub aion_collected: i32,
    pub stamina: i32,
    pub stamina_max: i32,
    pub stamina_regen_rate: i32,
}

impl Hero {
    pub fn new(
        base_stats: BaseStats,
        attributes: Attributes,
        aion_capacity: i32,
        aion_collected: i32,
    ) -> Self {
        Self {
            id: None,
            base_stats,
            inventory: None,
            attributes,
            retinue_slots: vec![],
            aion_capacity,
            resources: vec![],
            aion_collected,
            stamina: 100,
            stamina_max: 100,
            stamina_regen_rate: 1,
        }
    }
    pub fn regenerate_stamina(&mut self, res: RegionActionResult) {
        // set the self.stamina to number of seconds since last regionactionresult.created time and now
        // multiplied by self.stamina_regen_rate
        if let Some(created_time) = res.created_time {
            let now = chrono::Utc::now();
            let seconds = now.signed_duration_since(created_time).num_seconds() as i32;
            let stamina = seconds * self.stamina_regen_rate;
            // add to self.stamina only if it is less than self.stamina_max
            if self.stamina + stamina < self.stamina_max {
                self.stamina += stamina;
            } else {
                self.stamina = self.stamina_max;
            }
        }
    }
    pub fn add_resources(&mut self, new_resources: Vec<Resource>) {
        for new_resource in new_resources {
            let mut found = false;
            for existing_resource in &mut self.resources {
                if std::mem::discriminant(existing_resource) == std::mem::discriminant(&new_resource) {
                    // If the type of resource matches, add the values together
                    match (existing_resource, &new_resource) {
                        (Resource::Aion(ref mut value), Resource::Aion(new_value)) => {
                            *value += new_value;
                        }
                        (Resource::Valor(ref mut value), Resource::Valor(new_value)) => {
                            *value += new_value;
                        }
                        // ... handle other Resource variants similarly
                        _ => {}
                    }
                    found = true;
                    break;
                }
            }
            // If the resource type was not found in the existing resources, push it to the list
            if !found {
                self.resources.push(new_resource);
            }
        }
    }

    pub async fn update_stats(&mut self, loot_box: &TaskLootBox) -> Result<()> {
        match loot_box {
            TaskLootBox::Region(result) => {
                let xp = result.xp;
                self.gain_experience(xp);
                // find the resource enum type in the  self.resources and increase the amount by result.resources
                result.resources.iter().for_each(|result_resource| {
                    match result_resource {
                        Resource::Aion(amount) => {
                            self.aion_collected += amount;
                        }
                        Resource::
                    }
                })
                // Infra::repo().update_hero(&hero).await?;
            }
            TaskLootBox::Channel(result) => {
                let hero_id = result.hero_id.clone();
                let xp = result.xp;
                // let hero = Infra::repo().get_hero_by_id(&hero_id).await?;
                // let mut hero = hero.unwrap();
                // hero.gain_experience(xp);
                // Infra::repo().update_hero(&hero).await?
            }
        }
        Ok(())
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
