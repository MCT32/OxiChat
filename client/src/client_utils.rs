use irc::{IrcConnection, messages::{Message, Command}};

use crate::STATE;

pub fn on_message_received(msg: Message) {
    
    let binding = STATE;
    let mut state = binding.write().unwrap();

    state.chat.push(msg.to_string());

}

pub fn push_messages_received() {

    todo!()

}

pub async fn join_channel(connection: &mut IrcConnection, channel: String) {

    connection
        .send(Message {
            prefix: None,
            command: Command::Join(format!("#{}", channel).to_string()),
        })
        .await
        .unwrap();

}

pub async fn send_message(connection: &mut IrcConnection, prompt: String, _nickname: String, channel: String) {

    let message_input = prompt;
    if !message_input.is_empty() {
        connection
            .send(Message {
            prefix: None,
            command: Command::PrivMsg( 
                format!("{}", channel).to_string(),
                format!(":{}", message_input.to_string())
            ), }).await.unwrap();

    }   
}