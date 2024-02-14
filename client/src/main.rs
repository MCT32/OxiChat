mod config;
mod client_utils;
mod ui_utils;

use std::{io::{self, stdout, ErrorKind, Read, Stdout, Write}, string, sync::RwLock, thread, time::Duration};
use client_utils::send_message;
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{read, poll, Event, KeyCode, KeyModifiers, KeyEventKind};
use crossterm::style::{Print, SetBackgroundColor, SetForegroundColor, Color};
use crossterm::{execute, QueueableCommand};
use crossterm::cursor::{MoveTo};

use irc::{error, messages::{Command, Message}, IrcConnection};

use crate::ui_utils::{get_terminal_size};
use crate::config::{create_config};

const NERDROOM_ASCII: &str = include_str!("./ascii.txt");

pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize, //1:29:40
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
                                match &prompt[1..].trim() {
                                    &"conf" => { 
                                        // this will take 3 arguments; nickname, address, port. eg: '/conf Binkus irc.megacraftingtable.chat 6667"
                                        // would connect you to 'irc.megacraftingtable.net' on port '6667' using nickname 'Binkus.'

                                        let parts: Vec<&str> = prompt.split_whitespace().collect();
                                        if parts.len() == 4 {
                                            let nickname = parts[1];
                                            let address = parts[2];
                                            let port = parts[3].parse::<u16>().unwrap_or_default(); 
                                            let mut connection = config::create_config(nickname.to_string(), address.to_string(), port).await;
                                            // configured = true;
                                            chat.push(prompt.clone());
                                        } else {
                                            let error_msg = String::new();
                                            let error_msg = "Invalid arguments.";
                                            chat.push(error_msg.to_string());
                                        }
                                    }

                                    &"join" => {
                                        // will take one argument; channel. if you arent in a server, then this will display a message saying that you
                                        // are not currently connected to a server, will prompt you to enter one. 

                                        todo!() // execute join block, join channel
                                    }

                                    &"leave" => {
                                        todo!() // i dont know if theres a leave command yet | EDIT: there is. not implemented yet
                                    }

                                    &"quit" => {
                                        todo!() // will leave server.
                                    }

                                    _ => {
                                        todo!() // if invalid
                                    }

                                }
                            } else {
                                chat.push(prompt.clone());

                                if let Some(connection) = connection {
                                    send_message(&mut connection, prompt, nickname, channel).await;
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