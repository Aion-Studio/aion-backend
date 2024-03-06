use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::{repos::repo::Repo, services::impls::tasks::TaskManager};

pub struct Infrastructure {
    repo: Arc<Repo>,
    tasks: Arc<TaskManager>, // Add the TaskManager here
}

impl Infrastructure {
    fn new() -> Self {
        let repo = Arc::new(Repo::new());

        let tasks = Arc::new(TaskManager::new());

        Infrastructure { tasks, repo }
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
    pub fn initialize() {
        let mut infra_opt = INFRASTRUCTURE.lock().unwrap();
        if infra_opt.is_none() {
            *infra_opt = Some(Infrastructure::new());
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

    pub fn repo() -> Arc<Repo> {
        let instance = Self::instance();
        let infra_guard = instance.lock().unwrap();
        if let Some(infra) = &*infra_guard {
            infra.repo()
        } else {
            panic!("Infrastructure not initialized!");
        }
    }
}
