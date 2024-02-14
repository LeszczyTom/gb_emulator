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

    let width: usize = 500;
    let height: usize = 500;
    let image_size = width * height;

    let mut color: u8 = 0;

    loop {
        color = color.wrapping_add(1);

        for client in clients.lock().unwrap().values_mut() {
            let mut image_data: Vec<u8> = Vec::new();

            for _ in 0..image_size {
                image_data.append(&mut vec![color, color, color, 255]);
            }

            client.send_message(message::Message::Binary(image_data))?;
        }

        sleep(Duration::from_millis(33));
    }
}
