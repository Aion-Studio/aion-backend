use std::sync::{Arc, Mutex};

use prisma_client_rust::chrono::{self, DateTime, Duration, Local};
use rand::Rng;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use tracing::warn;
use uuid::Uuid;

use crate::{
    models::{hero::Hero, region::RegionName},
    services::traits::async_task::{BaseTask, Task, TaskExecReturn, TaskStatus},
};
use crate::configuration::get_explore_durations;
use crate::models::hero::{Attributes, BaseStats, Range};
use crate::models::region::HeroRegion;

use super::action_names::ActionNames;

#[derive(Clone, Debug)]
pub struct ExploreAction {
    base: BaseTask,
    pub hero: Hero,
    pub duration: Duration,
    pub region_name: RegionName,
    pub xp: i32,
    /// The discovery level increase for doing the exploration.
    /// This is a randomly generated integer between 1 and 5 for now
    ///
    pub discovery_level: f64,
    pub start_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    pub stamina_cost: i32,
}

impl Default for ExploreAction {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let hero = Hero::new(
            BaseStats {
                id: None,
                level: 1,
                xp: 0,
                damage: Range {
                    min: rng.gen_range(1..5),
                    max: rng.gen_range(5..10),
                },
                resilience: rng.gen_range(0..1),
                hit_points: 30,
                armor: rng.gen_range(0..=10),
            },
            Attributes {
                id: None,
                strength: rng.gen_range(1..20),
                agility: rng.gen_range(1..20),
                intelligence: rng.gen_range(1..20),
                exploration: rng.gen_range(1..20),
                crafting: rng.gen_range(1..20),
            },
            rng.gen_range(80..120),
            0,
        );
        ExploreAction::without_cost(hero, RegionName::Forest)
    }
}

impl Serialize for ExploreAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ExploreAction", 2)?;

        state.serialize_field("regionName", &self.region_name)?;
        state.serialize_field("heroId", &self.hero.id)?;
        state.serialize_field("heroName", &self.hero.name)?;

        let time_left = self.base.get_end_time().unwrap();

        let local_datetime: DateTime<Local> = time_left.with_timezone(&Local);
        state.serialize_field("endTime", &local_datetime)?;

        state.end()
    }
}

impl ExploreAction {
    pub fn new(hero: Hero, hero_region: HeroRegion, stamina_cost: i32) -> Option<Self> {
        let mut action = Self::without_cost(hero.clone(), hero_region.region_name.clone());
        action.stamina_cost = stamina_cost;

        if (hero.stamina - stamina_cost) < 0 {
            warn!(
                "hero stamina is {:?} but cost to do action is {:?}",
                hero.stamina, stamina_cost
            );
            return None;
        }

        Some(action)
    }

    pub fn without_cost(hero: Hero, region_name: RegionName) -> Self {
        let durations = get_explore_durations();
        let duration = *durations
            .0
            .get(&region_name)
            .clone()
            .to_owned()
            .unwrap_or(&Duration::minutes(1));

        Self {
            base: BaseTask::new(duration, hero.clone()),
            duration,
            hero,
            start_time: Arc::new(Mutex::new(None)),
            region_name: region_name.clone(),
            discovery_level: rand::thread_rng().gen_range(1..5) as f64,
            // random number between 15 and 30
            xp: rand::thread_rng().gen_range(15..30),
            stamina_cost: 0,
        }
    }
    // TODO: figure out if the costs are linear , baseline , or random per region
    pub fn get_stamina_cost(region_name: &RegionName, hero_discovery: f64) -> i32 {
        let discovery_range = match hero_discovery as i32 {
            0..=25 => (10, 12),
            26..=50 => (12, 15),
            51..=75 => (15, 20),
            _ => (25, 35),
        };

        match region_name {
            RegionName::Forest => {
                rand::thread_rng().gen_range(discovery_range.0..discovery_range.1)
            }
            RegionName::Yezer => {
                rand::thread_rng().gen_range(discovery_range.0 + 10..discovery_range.1 + 10)
            }
            RegionName::Buzna => {
                rand::thread_rng().gen_range(discovery_range.0 + 5..discovery_range.1 + 22)
            }
            RegionName::Dusane => {
                rand::thread_rng().gen_range(discovery_range.0 - 5..discovery_range.1 + 5)
            }
            RegionName::Lindon => {
                rand::thread_rng().gen_range(discovery_range.0..discovery_range.1 + 13)
            }
            RegionName::Emerlad => {
                rand::thread_rng().gen_range(discovery_range.0 + 15..discovery_range.1 + 30)
            }
            RegionName::Veladria => {
                rand::thread_rng().gen_range(discovery_range.0 + 20..discovery_range.1 + 35)
            }
        }
    }

    pub fn action_name(&self) -> ActionNames {
        ActionNames::Explore
    }

    pub fn calculate_boost_factor(&self, exploration: i32) -> f64 {
        let mut rng = rand::thread_rng();

        if exploration <= 10 {
            1.0
        } else {
            // Define the base and maximum possible boost factors
            let min_boost = 1.0;
            let max_boost = 1.4;

            // Map the exploration value to a range between 0 and 1
            let normalized_exploration = ((exploration - 10) as f64) / 40.0;

            // Calculate a base boost factor linearly interpolated between min_boost and max_boost based on exploration
            let base_boost = min_boost + (max_boost - min_boost) * normalized_exploration;

            // Introduce randomness: vary the final boost by up to +/- 5% of the current base boost
            let random_variation = rng.gen_range(-0.05..=0.05) * base_boost;
            let randomized_boost = base_boost + random_variation;

            // Ensure the boost is within the desired range
            round(randomized_boost.clamp(min_boost, max_boost), 2)
        }
    }
}

pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

impl Task for ExploreAction {
    fn execute(&self) -> TaskExecReturn {
        self.base.execute()
    }

    fn check_status(&self) -> TaskStatus {
        self.base.check_status()
    }

    fn start_now(&self) {
        self.base.start_now()
    }

    fn hero_id(&self) -> String {
        self.base.hero_id()
    }

    fn start_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.base.start_time()
    }

    fn task_id(&self) -> Uuid {
        self.base.task_id()
    }
    fn name(&self) -> String {
        ActionNames::Explore.to_string()
    }
}
