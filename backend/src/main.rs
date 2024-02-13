use client::Client;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use websocket_server::WebsocketServer;

mod client;
mod message;
mod websocket_server;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Clients = Arc<Mutex<HashMap<String, Client>>>;

fn main() -> Result<()> {
    let clients: Clients = Arc::new(Mutex::new(HashMap::<String, Client>::new()));
    let mut websocket_server: WebsocketServer = WebsocketServer::new(clients.clone());
    websocket_server.start("127.0.0.1:2794")?;

    loop {
        for client in clients.lock().unwrap().values_mut() {
            client.send_message(message::Message::Text(format!(
                "Sent at {:?}",
                std::time::SystemTime::now()
            )))?;
        }

        sleep(Duration::from_millis(33));
    }

    Ok(())
}
