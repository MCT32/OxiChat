use std::{io, sync::mpsc::Receiver};
use std::io::Write;
use std::sync::{Once, OnceLock};
use std::time::Duration;
use irc::{IrcConnection, messages::{Message, Command}};
use tokio::time::sleep;
use crossterm::{execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};

use crate::{State, STATE};

const TEXT: &str = include_str!("./ascii.txt");

pub fn on_message_received(message: Message) {

    let mut state = STATE.write().unwrap();

    state.messages.push(message);

    print_messages(&state.messages);

}

pub fn print_messages(messages: &Vec<Message>) {    
    print_ascii_art();
 
    for message in messages {

        let prefix = match message.prefix.as_ref() {
            Some(message) => message,
            None => ""  
        };

        match &message.command {
            Command::Notice(receiver, msg) => {
                println!("Notice from {}: {}", receiver, msg);
            }
            Command::Join(channel) => {
                println!("{} joined channel {}", prefix, channel);
            }
            command => {
                if let Command::Raw(command, params) = command.raw() {
                    println!("{} {} {}", prefix, command, params.join(" "));
                }
            }
    }
    }
}

pub fn get_input(prompt: &str) -> String {
    print!("\x1B[2J\x1B[1;1H");
    print_ascii_art();
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub async fn irc_client(connection: &mut IrcConnection, channel: String) {
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

pub async fn send_message(connection: &mut IrcConnection, channel: String) {
    print_ascii_art();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() {
            connection
                .send(Message {
                prefix: None,
                command: Command::PrivMsg( 
                    format!("#{}", channel).to_string(),
                    format!(":{}", input.to_string())
                ),
            })
        .await
        .unwrap();
            if input == "QUIT" {
                connection.quit().await.unwrap();
                break;
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
