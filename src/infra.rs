use std::sync::{Arc, Mutex};

use crate::{
    events::{
        dispatcher::{EventDispatcher, EventHandler},
        game::GameEvent,
    },
    prisma::PrismaClient,
    repos::region_repo::Repo,
    services::impls::tasks::TaskManager,
};
use lazy_static::lazy_static;

pub struct Infrastructure {
    dispatcher: EventDispatcher,
    repo: Arc<Repo>,
    tasks: Arc<TaskManager>, // Add the TaskManager here
}

impl Infrastructure {
    fn new(prisma_client: Arc<PrismaClient>) -> Self {
        let repo = Arc::new(Repo::new(prisma_client));

        let dispatcher = EventDispatcher::new();
        let tasks = Arc::new(TaskManager::new());

        Infrastructure {
            tasks,
            dispatcher,
            repo,
        }
    }

    pub fn dispatch(&self, args: GameEvent) {
        // Implement the dispatch logic here
        self.dispatcher.dispatch(args);
    }

    pub fn subscribe<E: EventHandler + 'static + Send + Sync>(
        &mut self,
        event_name: &str,
        handler: E,
    ) {
        let arc_handler = Arc::new(handler);
        self.dispatcher.subscribe(event_name, arc_handler);
    }

    pub fn repo(&self) -> Arc<Repo> {
        Arc::clone(&self.repo) // Just clone the Arc here
    }
}

lazy_static! {
    static ref INFRASTRUCTURE: Arc<Mutex<Option<Infrastructure>>> = Arc::new(Mutex::new(None));
}

pub struct Infra;

impl Infra {
    pub fn initialize(prisma_client: Arc<PrismaClient>) {
        let mut infra_opt = INFRASTRUCTURE.lock().unwrap();
        if infra_opt.is_none() {
            *infra_opt = Some(Infrastructure::new(prisma_client));
        }
    }

    fn instance() -> Arc<Mutex<Option<Infrastructure>>> {
        Arc::clone(&INFRASTRUCTURE)
    }

    pub fn tasks() -> Arc<TaskManager> {
        let instance = Self::instance();
        let infra_guard = instance.lock().unwrap();
        if let Some(infra) = &*infra_guard {
            Arc::clone(&infra.tasks)
        } else {
            panic!("Infrastructure not initialized!");
        }
    }

    pub fn dispatch(args: GameEvent) {
        let instance = Self::instance();
        let mut infra_guard = instance.lock().unwrap();
        if let Some(infra) = &mut *infra_guard {
            infra.dispatch(args);
        } else {
            panic!("Infrastructure not initialized!");
        }
    }

    pub fn repo() -> Arc<Repo> {
        let instance = Self::instance();
        let infra_guard = instance.lock().unwrap();
        if let Some(infra) = &*infra_guard {
            infra.repo()
        } else {
            panic!("Infrastructure not initialized!");
        }
    }

    pub fn subscribe(event_name: &str, handler: Arc<dyn EventHandler + Send + Sync>) {
        let instance = Self::instance();
        let mut infra_guard = instance.lock().unwrap();
        if let Some(infra) = &mut *infra_guard {
            infra.dispatcher.subscribe(event_name, handler);
        } else {
            panic!("Infrastructure not initialized!");
        }
    }
}
