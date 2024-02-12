use std::{net::TcpStream, thread};
use uuid::Uuid;
use websocket::OwnedMessage;

use crate::message::Message;

pub struct Client {
    id: Uuid,
}

impl Client {
    pub fn new(a: websocket::sync::Client<TcpStream>, id: Uuid) -> Self {
        println!("Client {} connected", id.to_string());
        let (mut ws_receiver, mut ws_sender) = a.split().unwrap();
        let (sender, receiver) = crossbeam_channel::unbounded::<Message>();

        thread::spawn(move || loop {
            if let Ok(message) = receiver.recv() {
                match message {
                    Message::Ping(ping) => {
                        ws_sender.send_message(&OwnedMessage::Pong(ping)).unwrap();
                    }
                    Message::Pong() => todo!(),
                    Message::Close => {
                        ws_sender.send_message(&OwnedMessage::Close(None)).unwrap();
                        println!("Client {} disconnected", id.to_string());
                        return;
                    }
                    Message::Text(text) => {
                        ws_sender.send_message(&OwnedMessage::Text(text)).unwrap()
                    }
                }
            }
        });

        thread::spawn(move || loop {
            if let Ok(message) = ws_receiver.recv_message() {
                match message {
                    OwnedMessage::Text(text) => sender.send(Message::Text(text)).unwrap(),
                    OwnedMessage::Binary(_) => todo!(),
                    OwnedMessage::Close(_) => sender.send(Message::Close).unwrap(),
                    OwnedMessage::Ping(ping) => sender.send(Message::Ping(ping)).unwrap(),
                    OwnedMessage::Pong(_) => todo!(),
                };
            };
        });

        Self { id }
    }
}
