use irc::messages::Message;

#[derive(Debug)]
pub struct Client {
    pub nickname: String,
    pub address: String,
    pub port: u16,
}

#[derive(Debug)]
pub enum Placeholder {
    PlaceholderA,
    PlaceholderB,
}

pub fn message_receiver(msg: Message) {
    println!("message receieved.")
}
