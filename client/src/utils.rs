use std::{io, sync::RwLock};
use std::io::Write;
use std::time::Duration;
use irc::{IrcConnection, messages::{Message, Command}};
use tokio::time::sleep;
use crossterm::{execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};
use crossterm::{QueueableCommand, queue};
use crate::STATE;
const TEXT: &str = include_str!("./ascii.txt");

pub fn on_message_received(message: Message) {
    let mut state = STATE.write().unwrap();

    state.messages.push(message);

    print_messages(&state.messages);
}

pub fn print_messages(messages: &Vec<Message>) {
    print_ascii_art();
    for message in messages {
        let prefix = match &message.prefix {
            Some(prefix) => {
                let nickname = prefix.split('!').next().unwrap_or("");
                format!("<{}>", nickname)
            }
            None => String::new(),
        };

        match &message.command {
            Command::PrivMsg(_, msg) => {
                print!("{} ", format!("\x1b[35m{}", prefix)); // uses ansi because crossterm is being fucky 
                println!("{}{}", msg.trim_start_matches(':'), "\x1b[0m"); 
            }
            Command::Join(channel) => {
                println!("{} joined channel {}", prefix, channel);
            }
            Command::Raw(command, params) => {
                println!("{} {} {}", prefix, command, params.join(" "));
            }
            _ => {} // ignore other command types for now 
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    print!("\x1B[2J\x1B[1;1H"); 
    print_ascii_art(); 
    print!("{}", prompt); 
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn get_message_input(prompt: &str) -> String {
    let (cols, rows) = terminal::size().expect("Failed to get terminal size");
    execute!(io::stdout(), cursor::MoveTo(cols + 1, rows)).unwrap();
    print!("{}", prompt); 
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


pub async fn irc_client(connection: &mut IrcConnection, channel: String) { // TODO this can be improved upon
    sleep(Duration::from_secs(2)).await;
    connection
        .send(Message {
            prefix: None,
            command: Command::Join(format!("#{}", channel).to_string()),
        })
        .await
        .unwrap();
    sleep(Duration::from_secs(2)).await;
}

pub async fn send_message(connection: &mut IrcConnection, channel: String, nickname: String) {
    print_ascii_art();
    loop {
        let prompt = format!("<{}> ", nickname).to_string();
        let input = get_message_input(&prompt);

        if !input.is_empty() {
            if input.starts_with('/') {
                match input.split_whitespace().next() {
                    Some(command) => {
                        match command {
                            "/join" => {
                                let params: Vec<&str> = input.split_whitespace().collect();
                                if params.len() >= 2 {
                                    let channel = params[1].to_string();
                                    connection.send(Message {
                                        prefix: None,
                                        command: Command::Join(channel.clone()),
                                    }).await.unwrap(); // doesnt really do much right now cause we auto join
                                } else {
                                    println!("ur gay i cant remember why i put this string here");
                                }
                            }
                            
                            // more commands to cum !

                            _ => println!("Invalid command"),
                        }
                    }
                    None => println!("Invalid command"),
                }
            } else {
                let message = Message {
                    prefix: None,
                    command: Command::PrivMsg(format!("#{}", channel).to_string(), format!(":{}", input.to_string())),
                };
                connection.send(message.clone()).await.unwrap();
                if input == "QUIT" {
                    break;
                }
                let mut state = STATE.write().unwrap();
                state.messages.push(message);
                print_messages(&state.messages);
            }
        }
    }
}


pub fn print_ascii_art() {  
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!("{} \n", TEXT)),
        ResetColor,
    ).unwrap();
    stdout.flush().unwrap();
}