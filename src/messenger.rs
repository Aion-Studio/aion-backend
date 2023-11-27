use tokio::sync::mpsc;

use crate::services::tasks::action_names::Command;

pub struct MessageManager {
    tx: mpsc::Sender<Command>,
}

impl MessageManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(10000);
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Handle incoming messages
                println!("Received message: {:?}", msg);
            }
        });

        MessageManager { tx }
    }

    pub fn tx_clone(&self) -> mpsc::Sender<Command> {
        self.tx.clone()
    }

    pub async fn send_message(
        &self,
        message: Command,
    ) -> Result<(), mpsc::error::SendError<Command>> {
        self.tx.send(message).await
    }
}
