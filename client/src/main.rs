mod config;
mod client_utils;

use std::{io::{stdout, Write}, sync::RwLock, thread, time::Duration};

use crossterm::event::{read, poll, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;
use crossterm::cursor::MoveTo;

use client_utils::{join_channel, send_message};

use irc::IrcConnection;

const NERDROOM_ASCII: &str = include_str!("./ascii.txt");

struct State {
    chat: Vec<String>
}

const STATE: RwLock<State> = RwLock::new(State {chat: Vec::new()});

pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize, 
}

fn chat_window(stdout: &mut impl Write, chat: &[String], boundary: Rect) {

    let n = chat.len();
    let m = n.checked_sub(boundary.h).unwrap_or(0);

    for (dy, line) in chat.iter().skip(m).enumerate() {
        stdout.queue(MoveTo(boundary.x as u16, (boundary.y + dy) as u16 )).unwrap();
        let bytes = line.as_bytes();
        stdout.write(bytes.get(0..boundary.w).unwrap_or(bytes)).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let mut stdout = stdout();

    let _ = terminal::enable_raw_mode().unwrap();
    let (mut w, mut h) = terminal::size().unwrap();
    let bar_char = "━";
    let mut bar = bar_char.repeat(w as usize);
    let mut prompt = String::new();
    let mut quit = false;

    let mut connection: Option<IrcConnection> = None; 
    let mut nickname = String::new(); 
    let mut channel = String::new(); 

    let mut configured = false;

    let mut chat = Vec::new();
    while !quit {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = bar_char.repeat(w as usize);
                }
                Event::Paste(data) => {
                    prompt.push_str(&data);
                }
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char(x) => {
                            if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                                quit = true;
                            } else {
                                prompt.push(x);
                            }
                        }
                        KeyCode::Enter => {
                            if prompt.starts_with('/') {
                                match &prompt.split_once(' ').unwrap().0 {
                                    &"/conf" => { 
                                        // this will take 3 arguments; nickname, address, port. eg: '/conf Binkus irc.megacraftingtable.chat 6667"
                                        // would connect you to 'irc.megacraftingtable.net' on port '6667' using nickname 'Binkus.'

                                        let parts: Vec<&str> = prompt.split_whitespace().collect();
                                        if parts.len() == 4 {
                                            nickname = parts[1].to_string(); // this cunt extracts the name out the fucka
                                            let address = parts[2]; // this cunt does the same thing for server addy
                                            let port = parts[3].parse::<u16>().unwrap_or_default();  // i cant remember why i unwrapped this but whatever
                                            let config = config::create_config(nickname.to_string(), address.to_string(), port).await;
                                            connection = Some(config.connect().await.unwrap());
                                            // configured = true;
                                            chat.push(prompt.clone());
                                        } else {
                                            let error_msg = "Invalid arguments.";
                                            chat.push(error_msg.to_string());
                                        }
                                    }

                                    &"/join" => {
                                        let parts: Vec<&str> = prompt.split_whitespace().collect();
                                        if parts.len() == 2 {
                                            let channel = parts[1].to_string();
                                            //join_channel(&mut connection.clone().unwrap(), channel).await;
                                        } else {
                                            let error_msg = "Invalid arguments.";
                                            chat.push(error_msg.to_string());
                                        }
                                    }

                                    &"/leave" => {
                                        todo!() // i dont know if theres a leave command yet | EDIT: there is. not implemented yet
                                    }

                                    &"/quit" => {
                                        todo!() // will leave server.
                                    }

                                    _ => {
                                        let error_msg = "Invalid command.";
                                        chat.push(error_msg.to_string());
                                    }

                                }
                            } else {
                                chat.push(prompt.clone());

                                if let Some(mut connection) = connection.clone() {
                                    send_message(&mut connection, prompt.clone(), nickname.clone(), channel.clone()).await;
                                }

                            }
                            prompt.clear();
                        }
                        KeyCode::Backspace => {
                            prompt.pop();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            stdout.queue(Clear(ClearType::All)).unwrap();
            chat_window(&mut stdout, &chat, Rect {
                x: 0,
                y: 0,
                w: w as usize,
                h: h as usize - 2,
            });

            stdout.queue(MoveTo(0, h - 2)).unwrap();
            stdout.write(bar.as_bytes()).unwrap();

            stdout.queue(MoveTo(0, h - 1)).unwrap();

            {
                let bytes = prompt.as_bytes();
                stdout.write(bytes.get(0..w as usize).unwrap_or(bytes)).unwrap();
            }

            stdout.flush().unwrap();

            thread::sleep(Duration::from_millis(33));
        }
    }
    terminal::disable_raw_mode().unwrap();
}