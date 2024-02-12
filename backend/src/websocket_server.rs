use std::thread;
use uuid::Uuid;
use websocket::sync::Server;

use crate::{client::Client, Clients};

pub struct WebsocketServer {
    clients: Clients,
}

impl WebsocketServer {
    pub fn new(clients: Clients) -> Self {
        Self { clients }
    }

    pub fn start(&mut self, addr: &str) {
        self._start_server(addr);
        print!("Start server !");
    }

    fn _start_server(&mut self, addr: &str) {
        let server = Server::bind(addr).unwrap();
        let clients = self.clients.clone();

        thread::spawn(move || {
            for request in server.filter_map(Result::ok) {
                if !request.protocols().contains(&"rust-websocket".to_string()) {
                    request.reject().unwrap();
                    return;
                }

                let id = Uuid::new_v4();
                let client = request.use_protocol("rust-websocket").accept().unwrap();
                clients
                    .lock()
                    .unwrap()
                    .insert(id.to_string(), Client::new(client, id));
            }
        });
    }
}
