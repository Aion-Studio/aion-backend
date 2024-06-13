extern crate lazy_static;
extern crate tokio;

use lazy_static::lazy_static;
use prisma_client_rust::chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::to_string;
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tracing::warn;

pub struct Logger;

#[derive(Serialize)]
struct LogMessage<T> {
    timestamp: DateTime<Utc>,
    value: T,
}

lazy_static! {
    static ref VECTOR_ADDRESS: Mutex<Option<SocketAddr>> = Mutex::new(None);
}

pub fn log<T: Serialize + Send>(value: T)
where
    T: 'static,
{
    Logger::log(value)
}

impl Logger {
    pub fn init(address: &str) -> Result<(), std::net::AddrParseError> {
        let parsed_addr: SocketAddr = address.parse()?;
        let mut addr = VECTOR_ADDRESS.lock().unwrap();
        *addr = Some(parsed_addr);
        Ok(())
    }

    pub fn log<T: Serialize + Send>(value: T)
    where
        T: 'static,
    {
        let addr = match VECTOR_ADDRESS.lock().unwrap().clone() {
            Some(a) => a,
            None => {
                warn!("Logger not initialized");
                return;
            }
        };

        tokio::spawn(async move {
            let log_message = LogMessage {
                timestamp: Utc::now(),
                value,
            };

            let serialized_value = match to_string(&log_message) {
                Ok(val) => val,
                Err(e) => {
                    warn!("Failed to serialize value for logging: {}", e);
                    return;
                }
            };

            let mut stream = match TcpStream::connect(addr).await {
                Ok(s) => s,
                Err(e) => {
                    warn!("Failed to connect to log server: {}", e);
                    return;
                }
            };

            let message = serialized_value + "\n";
            if let Err(e) = stream.write_all(message.as_bytes()).await {
                warn!("Failed to send log message: {}", e);
            }
        });
    }
}
