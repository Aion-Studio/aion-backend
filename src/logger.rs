extern crate lazy_static;
extern crate tokio;

use lazy_static::lazy_static;
// use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Mutex;
// use tokio::io::AsyncWriteExt;
// use tokio::net::TcpStream;

pub struct Logger;

lazy_static! {
    static ref VECTOR_ADDRESS: Mutex<Option<SocketAddr>> = Mutex::new(None);
}

impl Logger {
    pub fn init(address: &str) {
        let parsed_addr: SocketAddr = address.parse().unwrap();
        let mut addr = VECTOR_ADDRESS.lock().unwrap();
        *addr = Some(parsed_addr);
    }

    // pub async fn log(event: Value) -> Result<(), Box<dyn std::error::Error>> {
    //     let addr = {
    //         let locked_addr = VECTOR_ADDRESS.lock().unwrap();
    //         if let Some(a) = *locked_addr {
    //             a
    //         } else {
    //             return Err(Box::new(std::io::Error::new(
    //                 std::io::ErrorKind::NotConnected,
    //                 "Logger not initialized",
    //             )));
    //         }
    //     };
    //
    //     let mut stream = TcpStream::connect(addr).await?;
    //     let message = serde_json::to_string(&event)? + "\n"; // Newline delimited JSON
    //     stream.write_all(message.as_bytes()).await?;
    //     Ok(())
    // }
}
