use std::sync::{Arc, Mutex};

use prisma_client_rust::chrono::{self, DateTime, Duration, Local};
use rand::Rng;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use uuid::Uuid;

use crate::configuration::{get_explore_durations, ExploreDurations};
use crate::events::game::ActionNames;
use crate::models::resources::MaterialType;
use crate::{
    models::{hero::Hero, region::RegionName},
    services::traits::async_task::{BaseTask, Task, TaskExecReturn, TaskStatus},
};

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
    pub discovery_level: i32,
    pub start_time: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    pub stamina_cost: i32,
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
    pub fn new(hero: Hero, region_name: RegionName) -> Option<Self> {
        // Implement task creation logic...
        let action = Self::without_cost(hero.clone(), region_name.clone());
        let stamina_cost = get_stamina_cost(&region_name);

        if (hero.stamina - stamina_cost) < 0 {
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
            discovery_level: rand::thread_rng().gen_range(1..5),
            // random number between 15 and 30
            xp: rand::thread_rng().gen_range(15..30),
            stamina_cost: 0,
        }
    }

    pub fn get_material_reward(&self, discovery_lvl: f64) -> MaterialType {
        match self.hero.base_stats.level {
            1..=10 => MaterialType::get_common_rng(),
            11..=30 => MaterialType::get_maybe_rare(&self.hero, &self, discovery_lvl),
            31..=60 => MaterialType::get_maybe_epic(&self.hero, &self, discovery_lvl),
            _ => unreachable!(),
        }
    }

    pub fn action_name(&self) -> ActionNames {
        ActionNames::Explore
    }

    pub fn calculate_boost_factor(&self, exploration: i32) -> f64 {
        if exploration <= 10 {
            1.0
        } else {
            // Apply an exponential function where base_value = 10, max_value = 40, and growth_factor = 0.03
            let base_value = 10.0;
            let max_value = 40.0;
            let growth_factor = 0.03;

            // Calculate boost factor
            let boost: f64 = 1.0
                + ((max_value - base_value)
                    * (1.0 - (-growth_factor * (exploration as f64 - base_value)).exp()))
                .min(0.40);

            boost
        }
    }
}

fn get_stamina_cost(region_name: &RegionName) -> i32 {
    match region_name {
        RegionName::Forest => rand::thread_rng().gen_range(10..20),
        RegionName::Yezer => rand::thread_rng().gen_range(20..30),
        RegionName::Buzna => rand::thread_rng().gen_range(15..37),
        RegionName::Dusane => rand::thread_rng().gen_range(5..20),
        RegionName::Lindon => rand::thread_rng().gen_range(10..25),
        RegionName::Emerlad => rand::thread_rng().gen_range(25..45),
        RegionName::Veladria => rand::thread_rng().gen_range(30..50),
    }
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
