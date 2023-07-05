use crate::{
    models::{hero::Hero, region::TaskResult},
    types::AsyncResult,
};

use std::{error::Error, sync::Arc, pin::Pin, future::Future};
use tokio::sync::mpsc::{Receiver, Sender};

pub trait GameEngine {
    fn generate_hero(&self) -> AsyncResult<Hero, Box<dyn Error>>;

    fn result_channels(
        &self,
    ) -> Result<Sender<TaskResult>, Box<dyn Error>>;

    fn listen_for_results(
        self: Arc<Self>,
        rx: Receiver<TaskResult>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>>; 
}
