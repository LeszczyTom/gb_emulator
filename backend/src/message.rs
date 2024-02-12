pub enum Message {
    Ping(Vec<u8>),
    Pong(),
    Close,
    Text(String),
}
