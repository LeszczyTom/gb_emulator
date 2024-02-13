pub enum Message {
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close,
    Binary(Vec<u8>),
    Text(String),
}
