use crossterm::{QueueableCommand, queue, execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};
use std::{cmp, io::{stdout, Write}, str::Bytes, sync::RwLock, time::Duration};
use irc::{IrcConnection, messages::{Message, Command}};
use tokio::time::sleep;

pub fn on_message_received(msg: Message) {

    chat.push(msg);

}

pub fn push_messages_received() {

    todo!()

}

pub async fn send_message(connection: &mut IrcConnection, prompt: String, nickname: String, channel: String) {

    let message_input = prompt;
    if !message_input.is_empty() {
        connection
            .send(Message {
            prefix: None,
            command: Command::PrivMsg( 
                format!("#{}", channel).to_string(),
                format!(":{}", message_input.to_string())
            ), }).await.unwrap();

    }   
}