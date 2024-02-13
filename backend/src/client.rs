use crate::message::Message;
use crossbeam_channel::Sender;
use std::{net::TcpStream, thread};
use uuid::Uuid;
use websocket::OwnedMessage;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    sender: Sender<Message>,
}

impl Client {
    pub fn new(a: websocket::sync::Client<TcpStream>, id: Uuid) -> Result<Self> {
        println!("Client {} connected", id.to_string());
        let (mut ws_receiver, mut ws_sender) = a.split()?;
        let (sender, receiver) = crossbeam_channel::unbounded::<Message>();
        let (sender_clone, _) = (sender.clone(), receiver.clone());

        thread::spawn(move || loop {
            if let Ok(message) = receiver.recv() {
                match message {
                    Message::Ping(ping) => {
                        ws_sender.send_message(&OwnedMessage::Pong(ping)).unwrap();
                    }
                    Message::Pong(pong) => {
                        ws_sender.send_message(&OwnedMessage::Ping(pong)).unwrap()
                    }
                    Message::Close => {
                        ws_sender.send_message(&OwnedMessage::Close(None)).unwrap();
                        println!("Client {} disconnected", id.to_string());
                        return;
                    }
                    Message::Text(text) => {
                        ws_sender.send_message(&OwnedMessage::Text(text)).unwrap()
                    }
                    Message::Binary(value) => ws_sender
                        .send_message(&OwnedMessage::Binary(value))
                        .unwrap(),
                }
            }
        });

        thread::spawn(move || loop {
            if let Ok(message) = ws_receiver.recv_message() {
                match message {
                    OwnedMessage::Text(text) => sender.send(Message::Text(text)).unwrap(),
                    OwnedMessage::Binary(value) => sender.send(Message::Binary(value)).unwrap(),
                    OwnedMessage::Close(_) => sender.send(Message::Close).unwrap(),
                    OwnedMessage::Ping(ping) => sender.send(Message::Ping(ping)).unwrap(),
                    OwnedMessage::Pong(pong) => sender.send(Message::Pong(pong)).unwrap(),
                };
            };
        });

        Ok(Self {
            id,
            sender: sender_clone,
        })
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        self.sender.send(message)?;

        Ok(())
    }
}
