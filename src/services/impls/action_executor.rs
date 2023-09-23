// use std::{future::Future, pin::Pin, sync::Arc};
//
// use rand::Rng;
//
// use crate::{
//     models::{
//         hero::{Attributes, BaseStats, Hero, Range},
//         region::HeroRegion,
//     },
//     prisma::PrismaClient,
//     repos::game_engine_repo::GameEngineRepo,
//     types::AsyncResult, events::{dispatcher::EventDispatcher, game::{GameEvent, TaskLootBox}}, infra::Infra,
// };
// use flume::{unbounded, Receiver, Sender};
//
// use crate::repos::region_repo::RegionRepo;
// use crate::services::impls::hero_service::ServiceHeroes;
// use crate::services::impls::tasks::TaskManager;
// use crate::services::tasks::explore::ExploreAction;
// use crate::services::traits::async_task::TaskError;
// use tracing::{error, info};
//
// #[derive(Clone, Debug)]
// pub struct ActionExecutor {
//     pub result_sender: Sender<TaskLootBox>,
//     pub result_broadcast_receiver: Receiver<TaskLootBox>,
//     pub task_sender: Sender<GameEvent>,
//     repo: Arc<GameEngineRepo>,
//     hero_service: Arc<ServiceHeroes>,
//     region_repo: Arc<RegionRepo>,
//     pub scheduler: Arc<TaskManager>,
// }
//
// impl ActionExecutor {
//     pub fn new(prisma: Arc<PrismaClient>) -> Arc<Self> {
//         // channel for other crates signaling their results to ActionExecutor
//         let (result_sender, result_receiver) = unbounded();
//         // Entry point for other crates to send tasks to ActionExecutor
//         let (task_sender, task_receiver) = unbounded();
//         // channel for ActionExecutor to send results to callers
//         let (result_broadcast_sender, result_broadcast_receiver) = unbounded();
//
//         let scheduler = Arc::new(TaskManager::new());
//         let repo = Arc::new(GameEngineRepo::new(prisma.clone()));
//         let hero_service = Arc::new(ServiceHeroes::new(prisma.clone()));
//
//         let region_repo = Arc::new(RegionRepo::new(prisma.clone()));
//         let service = Arc::new(Self {
//             result_sender,
//             result_broadcast_receiver,
//             task_sender,
//             repo,
//             region_repo,
//             hero_service,
//             scheduler,
//         });
//
//         // other fields
//
//         let service_clone = Arc::clone(&service);
//         tokio::spawn(async move {
//             service_clone
//                 .listen_for_results(result_receiver, result_broadcast_sender)
//                 .await;
//         });
//
//         let service_clone = Arc::clone(&service);
//         tokio::spawn(async move {
//             service_clone.handle_tasks(task_receiver).await;
//         });
//         // Spawn tasks
//         service
//     }
//
//     async fn handle_tasks(self: Arc<Self>, receiver: Receiver<GameEvent>) {
//         while let Ok(task) = receiver.recv_async().await {
//             let self_clone = self.clone();
//             self_clone.handle_task(task).await;
//         }
//     }
//
//     fn handle_task(self: Arc<Self>, task: GameEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> {
//         Box::pin(async move {
//             let (task_done_sender, task_done_receiver) = unbounded();
//             info!("Scheduling task: {:?}", task);
//             match self.scheduler.schedule(task, task_done_sender) {
//                 Ok(_) => {
//                     while let Ok(task) = task_done_receiver.recv_async().await {
//                         info!("Task completed: {:?}", task);
//                         let self_clone = self.clone();
//                         tokio::spawn(async move {
//                             self_clone.handle_task_completed(task).await;
//                         });
//                     }
//                 }
//                 Err(err) => {
//                     error!("Error scheduling task: {}", err);
//                 }
//             }
//         })
//     }
//
//     // Generate result
//     // Send notifications
//     // Update any state needed
//     // Metrics?
//     async fn handle_task_completed(&self, task: GameEvent) {
//         match task {
//             GameEvent::Exploration(action) => {
//                 Infra::dispatch(GameEvent::ExploreCompleted(action.clone()));
//                 if let Err(err) = self.explore_action_complete(action).await {
//                     error!("Error handling exploration: {}", err);
//                 }
//             }
//             GameEvent::HeroExplores(_) => todo!(),
//             GameEvent::ExploreCompleted(_) => todo!(),
//             GameEvent::LootBoxCreated(_) => todo!(),
//         }
//     }
//
//     async fn explore_action_complete(&self, action: ExploreAction) -> Result<(), TaskError> {
//         info!("Exploration action complete: {:?}", action);
//
//         let self_clone = self.clone();
//
//         let hero_id = action.clone().hero.id.unwrap();
//         // Stamina cost
//         self.repo
//             .deduct_stamina(&hero_id, action.stamina_cost)
//             .await?;
//
//         // Get loot reward
//         match self.generate_loot_box(&TaskAction::Explore(action)).await {
//             Ok(task_loot_box) => {
//                 // Increase  HeroRegion
//                 let discovery_level_increase = match &task_loot_box {
//                     TaskLootBox::Region(result) => result.discovery_level_increase,
//                 };
//                 self.region_repo
//                     .update_hero_region_discovery_level(
//                         &hero_id,
//                         discovery_level_increase.round() as i32,
//                     )
//                     .await?;
//                 // send lootbox result to results channel
//                 match self_clone.result_sender.send(task_loot_box) {
//                     Ok(_) => Ok(()),
//                     Err(err) => Err(TaskError::RepoError(err.to_string())),
//                 }
//             }
//             Err(_) => todo!(),
//         }
//     }
//
//     async fn generate_loot_box(&self, action: &TaskAction) -> Result<TaskLootBox, TaskError> {
//         match action {
//             TaskAction::Explore(action) => {
//                 let hero = &action.hero;
//                 let hero_id = hero.id.as_ref().unwrap();
//                 let boost_factor =
//                     self.calculate_boost_factor(action.hero.attributes.exploration.clone());
//                 let result = RegionActionResult {
//                     xp: (action.xp as f64 * boost_factor) as i32,
//                     hero_id: hero_id.clone(),
//                     resources: vec![],
//                     discovery_level_increase: (action.discovery_level as f64 * boost_factor),
//                     created_time: None,
//                 };
//
//                 Ok(TaskLootBox::Region(result))
//             }
//         }
//     }
//     pub fn calculate_boost_factor(&self, exploration: i32) -> f64 {
//         if exploration <= 10 {
//             1.0
//         } else {
//             // Apply an exponential function where base_value = 10, max_value = 40, and growth_factor = 0.03
//             let base_value = 10.0;
//             let max_value = 40.0;
//             let growth_factor = 0.03;
//
//             // Calculate boost factor
//             let boost: f64 = 1.0
//                 + ((max_value - base_value)
//                     * (1.0 - (-growth_factor * (exploration as f64 - base_value)).exp()))
//                 .min(0.40);
//
//             boost
//         }
//     }
//
//     pub fn result_channels(&self) -> Result<Sender<TaskLootBox>, Box<dyn std::error::Error>> {
//         Ok(self.result_sender.clone())
//     }
//
//     pub fn listen_for_results(
//         &self,
//         rx: Receiver<TaskLootBox>,
//         broadcast_sender: Sender<TaskLootBox>,
//     ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
//         let self_clone = self.clone();
//         Box::pin(async move {
//             /* Here we add all the cases for every type of action and
//              * update the state of heroes and data models accordingly.
//              *
//              *
//              *
//              */
//             while let Ok(result) = rx.recv_async().await {
//                 match result {
//                     TaskLootBox::Region(result) => {
//                         broadcast_sender
//                             .send(TaskLootBox::Region(result.clone()))
//                             .unwrap();
//                         if let Err(err) = self_clone
//                             .repo
//                             .clone()
//                             .store_region_action_result(result)
//                             .await
//                         {
//                             eprintln!("Error storing region action result: {}", err);
//                             return;
//                         }
//                     }
//                 }
//             }
//         })
//     }
//
//     pub fn generate_hero(&self) -> AsyncResult<Hero, Box<dyn std::error::Error>> {
//         Box::pin(async {
//             let mut rng = rand::thread_rng();
//
//             let hero = Hero::new(
//                 BaseStats {
//                     id: None,
//                     level: 1,
//                     xp: 0,
//                     damage: Range {
//                         min: rng.gen_range(1..5),
//                         max: rng.gen_range(5..10),
//                     },
//                     hit_points: rng.gen_range(90..110),
//                     mana: rng.gen_range(40..60),
//                     armor: rng.gen_range(5..15),
//                 },
//                 Attributes {
//                     id: None,
//                     strength: rng.gen_range(1..20),
//                     resilience: rng.gen_range(1..20),
//                     agility: rng.gen_range(1..20),
//                     intelligence: rng.gen_range(1..20),
//                     exploration: rng.gen_range(1..20),
//                     crafting: rng.gen_range(1..20),
//                 },
//                 rng.gen_range(80..120),
//                 0,
//             );
//
//             Ok(hero)
//         })
//     }
// }
