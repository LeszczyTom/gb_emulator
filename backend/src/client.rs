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
    pub fn new(ws_client: websocket::sync::Client<TcpStream>, id: Uuid) -> Result<Self> {
        println!("Client {} connected", id.to_string());
        let (mut ws_receiver, mut ws_sender) = ws_client.split()?;
        let (sender, receiver) = crossbeam_channel::unbounded::<Message>();
        let (sender_clone, _) = (sender.clone(), receiver.clone());

        thread::spawn(move || loop {
            if let Ok(message) = receiver.recv() {
                match message {
                    Message::Ping(ping) => {
                        if let Err(err) = ws_sender.send_message(&OwnedMessage::Pong(ping)) {
                            println!("{err}");
                        }
                    }
                    Message::Pong(pong) => {
                        if let Err(err) = ws_sender.send_message(&OwnedMessage::Ping(pong)) {
                            println!("{err}");
                        }
                    }
                    Message::Close => {
                        if let Err(err) = ws_sender.send_message(&OwnedMessage::Close(None)) {
                            println!("{err}");
                        } else {
                            println!("Client {} disconnected", id.to_string());
                        }
                    }
                    Message::Text(text) => {
                        if let Err(err) = ws_sender.send_message(&OwnedMessage::Text(text)) {
                            println!("{err}");
                        }
                    }
                    Message::Binary(value) => {
                        if let Err(err) = ws_sender.send_message(&OwnedMessage::Binary(value)) {
                            println!("{err}");
                        }
                    }
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
