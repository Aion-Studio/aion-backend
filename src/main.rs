use aion_server::configuration::get_configuration;
use aion_server::startup::Application;
use aion_server::telemetry::{get_subscriber, init_subscriber};
use tokio::task::JoinError;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("idle_rpg".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select! {
        o = application_task => report_exit("API", o),
    };

    Ok(())
}

fn report_exit(
    task_name: &str,
    outcome: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, JoinError>,
) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
