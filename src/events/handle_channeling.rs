use serde_json::json;
use tracing::info;

use crate::{
    configuration::ChannelDurations,
    logger::Logger,
    services::{
        tasks::{
            action_names::{Responder, TaskAction},
            channel::ChannelingAction,
        },
        traits::async_task::Task,
    },
};

use crate::infra::Infra;

#[derive(Debug, Clone)]
pub struct ChannelingHandler {}

impl ChannelingHandler {
    pub fn hero_channels(
        hero_id: String,
        leyline_name: String,
        durations: ChannelDurations,
        resp: Responder<()>,
    ) {
        tokio::spawn(async move {
            let action = Infra::repo()
                .get_hero(hero_id)
                .await
                .map(|hero| ChannelingAction::new(hero, &leyline_name, &durations))
                .unwrap_or_else(|_| None);

            match action {
                Some(action) => {
                    info!("Starting channeling ...");
                    action.start_now();
                    Infra::tasks().schedule_action(TaskAction::Channel(action));
                    let _ = resp.send(Ok(()));
                }
                None => {
                    let _ = resp.send(Err(anyhow::Error::msg("Couldn't start channeling")));
                }
            }
        });
    }

    pub fn channel_completed(action: ChannelingAction) {
        Logger::log(
            json!({"name": action.name(),"hero_id": action.hero_id(), "leyline":action.leyline }),
        )
    }
}
