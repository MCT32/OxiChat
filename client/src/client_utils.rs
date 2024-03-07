use std::{fmt::Write, ptr::null, time::Duration};

use crossterm::{queue, terminal::ClearType};
use futures::channel::oneshot::channel;
use irc::{config::IrcConfig, messages::{Command, Message}, IrcConnection};
use crossterm::{terminal::Clear, QueueableCommand};
use crossterm::cursor::MoveTo;
use irc::error;

use tokio::time::{sleep, Sleep};

use crate::{config, ui_utils::Screen, ChatsRef, CHATS};

#[derive(Clone)]
pub struct Client {
    pub nickname: String,
    pub address: String,
    pub port: u16,

    pub channel: String,

    pub connection: Option<IrcConnection>
}

impl Client { // these are methods
    pub async fn edit_client_config(&mut self, prompt: String) {

        let parts: Vec<&str> = prompt.split_whitespace().collect();
            if parts.len() == 4 {
                let new_nickname = parts[1].to_string(); // this cunt extracts the name out the fucka
                let new_address = parts[2].to_owned(); // this cunt does the same thing for server addy
                let new_port = parts[3].parse::<u16>().unwrap_or_default();  // i cant remember why i unwrapped this but whatever

                self.nickname = new_nickname;
                self.address = new_address;
                self.port = new_port;

                // boinkus
            } else {
                let error_msg = "Invalid arguments.";
                vector_vendor(error_msg.to_string());
            }
    }
    pub async fn edit_connection(&self, mut irc_configuration: IrcConfig, ) -> IrcConfig {
        irc_configuration = config::create_irc_config(self.clone()).await; 
        return irc_configuration;
    }
}

impl Client { // these are associated functions
    pub fn default_config(arguments: Vec<String>) -> Self {
        Self {
            nickname: arguments[1].clone(),
            address: arguments[2].clone(),
            port: arguments[3].parse().unwrap(),

            channel: "test".to_string(),

            connection: None
        }
    }
    pub async fn connect_command(irc_configuration: IrcConfig) -> IrcConnection {
        let connection = irc_configuration.connect().await.unwrap();
        connection
    }
    pub async fn join_command(&self, prompt: String) {
        let parts: Vec<&str> = prompt.split_whitespace().collect();
        if parts.len() == 2 {
            let channel = parts[1].to_string();

            sleep(Duration::from_secs(1)).await;
            self.connection.clone().unwrap()
                .send(Message {
                    prefix: None,
                    command: Command::Join(format!("#{}", channel).to_string()),
                })
                .await
                .unwrap();
            sleep(Duration::from_secs(1)).await;
        } else {
            let error_msg = "Invalid arguments.";
            vector_vendor(error_msg.to_string());
        }
    }
    pub async fn part_command() {
        todo!()
    }
    pub async fn quit_command() {

    }
    pub async fn send_message(client_configuration: Client, prompt: String) {
        client_configuration.connection.unwrap()
                .send(Message {
                prefix: None,
                command: Command::PrivMsg( 
                    format!("{}", client_configuration.channel).to_string(),
                    format!(":{}", prompt.to_string())
                ), }).await.unwrap();
                sleep(Duration::from_millis(500)).await;
    }
}


pub fn message_receiver(msg: Message) {

    vector_vendor(msg.to_string());

}

pub fn vector_vendor(message: String) { // this is a method to push whatever is passed to it to the CHATS vector.
    let mut chats = CHATS.write().unwrap();
    chats.push(message);
}