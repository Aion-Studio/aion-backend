use aion_server::configuration::get_configuration;
use aion_server::webserver::Application;
use tokio::task::JoinError;

use derive_more::{Display, Error};
use tracing::{error, info};
use tracing::log::warn;

extern crate aion_server;

#[allow(unused)]
#[derive(Debug, Display, Error)]
enum ApplicationError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "not found error on hero id: {}", field)]
    BadClientData { field: String },

    #[display(fmt = "timeout")]
    Timeout,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if !aion_server::tracing_subscribe() {
        warn!("no tracing subscriber");
    }
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    tokio::select! {
        o = application_task => report_exit("API", o),
    }
    ;

    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            error!("{} has panicked: {:?}", task_name, e)
        }

        Err(e) => {
            error!("{} has panicked: {:?}", task_name, e)
        }
    }
}
