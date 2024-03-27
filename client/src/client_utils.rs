use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::terminal;
use irc::*; use crate::errors::argument_error;
// apparantly this doesnt import everything, i dont understand the fucking crate system
use crate::CHATS;
use crate::ui_utils::*;

use std::process::exit;
use std::{fmt::Write, ptr::null, time::Duration};

use irc::{config::IrcConfig, messages::{Command, Message}, IrcConnection}; // this might be redundant

use crossterm::{terminal::Clear, QueueableCommand};
use crossterm::{queue, terminal::ClearType};
use crossterm::cursor::MoveTo;

use tokio::time::{sleep, Sleep}; // fucking fuck this library

#[derive(Clone)]
pub struct Client { // this sucks
    pub nickname: String,
    pub address: String,
    pub port: u16,

    pub channel: String,

    pub connection: Option<IrcConnection>
}

pub struct  Faculties {
    pub client: Option<Client>,
    pub client_screen: Option<Client_Screen>,
    pub irc_configuration: Option<IrcConfig>
}

impl Client { // configuration methods

    pub fn default_client_configuration(args: Vec<String>) -> Self {
        let nickname = args[1].clone();
        let address = args[2].clone();
        let port = args[3].parse().expect("ERROR: PORT MUST BE A VALID INTEGER");

        Client {
            nickname,
            address,
            port,
            channel: "test".to_string(),
            connection: None,
        }
    }
}

impl Client { // connection methods. this whole impl block is bad
    pub async fn connect_command(mut irc_configuration: & mut IrcConfig) -> IrcConnection {
        let connection = irc_configuration.connect().await.unwrap();
        connection
    }
    pub async fn join_command(&self, client_screen: &Client_Screen) {
        let parts: Vec<&str> = client_screen.input.split_whitespace().collect();
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
        todo!()
    }
    pub async fn send_message(client_configuration: Client, prompt: String) {
        client_configuration.connection.unwrap()
                .send(Message {
                prefix: None,
                command: Command::PrivMsg( 
                    format!("{} ", client_configuration.channel).to_string(),
                    format!(":{}", prompt.to_string())
                ), }).await.unwrap();
                sleep(Duration::from_millis(500)).await;
    }
}

impl Faculties {
    pub async fn create(client: Client, client_screen: Client_Screen, irc_configuration: IrcConfig) -> Self {
        Faculties {
            client: Some(client),
            client_screen: Some(client_screen),
            irc_configuration: Some(irc_configuration)
        }
    }
}

pub fn check_args_valid(args: &Vec<String>) -> bool { // Checks if startup arguments are valid

    let rook: u16 = args[3].parse().expect("ERROR: PORT MUST BE A VALID INTEGER"); // cum

    if args.len() == 4 {
        println!("ARGUMENTS {}, {}, {},  ARE VALID", args[1], args[2], args[3]);
        return true;
    } else {
        deinitialize_screen();
        argument_error();
        return false;
    }
}

pub async fn command_lexer(client_screen: &mut Client_Screen, client: &mut Client, mut irc_config: &mut IrcConfig) { // this really sucks but idk what to do about it
    match &client_screen.input.split(' ').next().unwrap() {
        &"/conf" => {
           // this command is fucking retarded.
        }

        &"/connect" => {
            client.connection = Some(Client::connect_command(&mut irc_config).await);
        }

        &"/join" => {
            
            Client::join_command(&client, &client_screen).await;
        }

        &"/leave" => {
            todo!() // i dont know if theres a leave command yet | EDIT: there is. not implemented yet
        }

        &"/quit" => {
            todo!() // will leave server.
        }

        _ => {
            let error_msg = "Invalid command.";
            vector_vendor(error_msg.to_string());
        }
    } 
}

pub async fn key_handler(mut client_screen: &mut Client_Screen, mut client: &mut Client, mut irc_config: &mut IrcConfig, event: KeyEvent) { // KEY HANDLER takes an event as an input and does something with it.
    match event.code {
        KeyCode::Char(x) => {
            if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) { // ctrl c my beloved <3
                exit(0); 
            } else {
                client_screen.input.push(x);
            }
        } // This just adds the character to the input string.

        KeyCode::Enter => {
            if client_screen.input.starts_with("/") {
                command_lexer(client_screen, &mut client, &mut irc_config).await;
            } else {
                todo!() // this needs to just send the message. i dont need to append it to the chat vector.
            }
        }

        _ => {
            println!("
            ERROR: INVALID KEYCODE!\n...somehow. Good job. I have no idea how you did this.
            ");
            deinitialize_screen();
        }
    }

}

pub fn message_receiver(msg: Message) {
    vector_vendor(msg.to_string());
}

pub fn vector_vendor(msg: String) {
    let mut chats = CHATS.write().unwrap();
    chats.push(msg);
}

pub fn kill_all() {
    let _ = terminal::disable_raw_mode();
    exit(0)
}