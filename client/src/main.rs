mod client_utils;
mod ui_utils;
mod config;

use client_utils::*;
use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;
use irc::{config::IrcConfig, error::IrcConnectError, IrcConnection};
use ui_utils::Screen;

use std::{env, io::{self, stdin, stdout, Write}, process, string, sync::RwLock, thread, time::Duration};
use std::sync::{Mutex, Arc};
use std::process::exit;

use futures::{FutureExt, StreamExt};

const NERDROOM_ASCII: &str = include_str!("./ascii.txt");

pub type ChatsRef = Arc<RwLock<Vec<String>>>;

lazy_static::lazy_static! {
    pub static ref CHATS: ChatsRef = Arc::new(RwLock::new(Vec::new()));
}

#[tokio::main]
pub async fn main() {

    // INITIALIZE ! {

    let mut stdout = stdout();

    let _ = crossterm::terminal::enable_raw_mode();
    stdout.queue(Clear(ClearType::All)).unwrap();

    let (mut h, mut w) = terminal::size().expect("Failed to get terminal size!");
    
    let bar_char = "█";

    let mut client_screen = Screen {
        stdout: stdout,
        h: h,
        w: w,
        chats: CHATS.clone(),
        prompt: String::new(),
        bar_char: bar_char.to_string(),
        bar: bar_char.repeat(w as usize)
    };

    let startup_args: Vec<String> = env::args().collect(); // collects the arguments passed when the fucka was launched
    if startup_args.len() == 4 { // TODO: REMOVE CHANNEL FROM STARTUP ARGS [len() == 4]
        {}
    } else {
        println!("Invalid arguments. Correct usage: \n'NerdRoom <NICKNAME> <SERVERADDRESS> <PORT>'");
        process::exit(0)
    }
    
    let mut client_configuration: Client = Client::default_config(startup_args.clone()); // initializes the client config with wtv args were passed on strt
    let mut irc_configuration: IrcConfig = config::create_irc_config(client_configuration.clone()).await;        // initializes irc config with client config
    // INITIALIZE ! }
    
    let mut reader = EventStream::new();

    loop { // main event loop.

        Screen::render_all(&mut client_screen);

        let event = reader.next().await.unwrap().unwrap();
        match event {
            Event::Resize(nw, nh) => {
                client_screen.w = nw;
                client_screen.h = nh;
                client_screen.bar = bar_char.repeat(w as usize); 
            }
            Event::Paste(data) => {
                client_screen.prompt.push_str(&data);
            }
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char(x) => {
                        if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {

                            break;
                        } else {
                            client_screen.prompt.push(x);
                        }
                    }
                    KeyCode::Enter => {
                         if client_screen.prompt.starts_with("/") {
                            match &client_screen.prompt.split(' ').next().unwrap() {
                                &"/conf" => {
                                    // this will take 3 arguments; nickname, address, port. eg: '/conf Binkus irc.megacraftingtable.chat 6667"
                                    // would connect you to 'irc.megacraftingtable.net' on port '6667' using nickname 'Binkus.'
                                    Client::edit_client_config(&mut client_configuration, client_screen.prompt.clone()).await;
                                    irc_configuration = Client::edit_connection(&client_configuration, irc_configuration.clone()).await;
                                }
                        
                                &"/connect" => {
                                    client_configuration.connection = Some(Client::connect_command(irc_configuration.clone()).await);
                                }
                        
                                &"/join" => {
                                     Client::join_command(&client_configuration.clone(), client_screen.prompt.clone()).await;
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
                            if client_configuration.connection.is_some() {
                                Client::send_message(client_configuration.clone(), client_screen.prompt.clone()).await;
                            }
                        }

                        vector_vendor(client_screen.prompt.clone());
                        client_screen.prompt.clear();
                    }
                    KeyCode::Backspace => {
                        client_screen.prompt.pop();
                    }
                    _ => {
                    }
                }
            }
            _ => {
            }
        }

        client_screen.stdout.queue(Clear(ClearType::All)).unwrap();
        Screen::render_all(&mut client_screen);
    }

    terminal::disable_raw_mode().unwrap();
        
}