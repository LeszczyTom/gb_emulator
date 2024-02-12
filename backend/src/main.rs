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

type Clients = Arc<Mutex<HashMap<String, Client>>>;

fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::<String, Client>::new()));
    let mut websocket_server: WebsocketServer = WebsocketServer::new(clients.clone());
    websocket_server.start("127.0.0.1:2794");

    loop {
        sleep(Duration::from_secs(2));
        println!("Sleep")
    }
}
