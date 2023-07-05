use std::sync::Arc;

use actix_web::web::Data;
use prisma_client_rust::QueryError;

use crate::models::hero::{Attributes, BaseStats, Follower, Inventory, Item, Range, RetinueSlot};
use crate::prisma::{attributes, base_stats, follower, hero, inventory, item, retinue_slot};
use crate::{models::hero::Hero, prisma::PrismaClient};

#[derive(Clone)]
pub struct HeroRepo {
    prisma: Arc<Data<PrismaClient>>,
}

impl HeroRepo {
    pub fn new(prisma: Arc<Data<PrismaClient>>) -> Self {
        Self { prisma }
    }
    pub async fn get_hero(&self, hero_id:String) ->Result<Hero, QueryError>{
        let hero = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero_id))
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .exec()
            .await?;
        Ok(hero.unwrap().into())
    }
    pub async fn create(&self, new_hero: Hero) -> Result<Hero, QueryError> {
        // Use Prisma to create a new Hero in the database
        // Convert the resulting record into a Hero struct and return it
        // ...
        let base_inventory = self.prisma.inventory().create(vec![]).exec().await.unwrap();

        let base_stats = self
            .prisma
            .base_stats()
            .create(
                new_hero.base_stats.level,
                new_hero.base_stats.xp,
                new_hero.base_stats.damage.min,
                new_hero.base_stats.damage.max,
                new_hero.base_stats.hit_points,
                new_hero.base_stats.mana,
                new_hero.base_stats.armor,
                vec![],
            )
            .exec()
            .await
            .unwrap();

        let base_attributes = self
            .prisma
            .attributes()
            .create(
                new_hero.attributes.strength,
                new_hero.attributes.resilience,
                new_hero.attributes.agility,
                new_hero.attributes.intelligence,
                new_hero.attributes.exploration,
                new_hero.attributes.crafting,
                vec![],
            )
            .exec()
            .await
            .unwrap();

        let result = self
            .prisma
            .hero()
            .create(
                new_hero.aion_capacity,
                new_hero.aion_collected,
                base_stats::id::equals(base_stats.clone().id),
                attributes::id::equals(base_attributes.clone().id),
                inventory::id::equals(base_inventory.clone().id),
                vec![],
            )
            .with(hero::base_stats::fetch())
            .with(hero::attributes::fetch())
            .with(hero::inventory::fetch())
            .exec()
            .await?;
        
        Ok(result.into())
    }
    pub async fn update_level(&self, hero: Hero) -> Result<Hero, QueryError> {
        let updated_base_stats = self
            .prisma
            .base_stats()
            .update(
                base_stats::id::equals(hero.base_stats.id.as_ref().unwrap().clone()),
                vec![base_stats::level::set(hero.base_stats.level + 1)],
            )
            .exec()
            .await;

        let updated_hero = self
            .prisma
            .hero()
            .find_unique(hero::id::equals(hero.get_id()))
            .with(hero::base_stats::fetch())
            .exec()
            .await?;

        assert_eq!(
            updated_base_stats.as_ref().unwrap().level,
            updated_hero
                .as_ref()
                .unwrap()
                .base_stats
                .as_ref()
                .unwrap()
                .level,
            "Base stats should be updated on the hero in the database"
        );

        Ok(updated_hero.unwrap().into())
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

        Self {
            id: Some(data.id),
            base_stats,
            attributes,
            inventory: Some(inventory),
            retinue_slots,
            aion_capacity: data.aion_capacity,
            aion_collected: data.aion_collected,
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
