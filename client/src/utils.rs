use std::{io, sync::mpsc::Receiver};

use std::time::Duration;
use irc::{IrcConnection, messages::{Message, Command}};
use tokio::time::sleep;

const TEXT: &str = include_str!("./ascii.txt");

pub fn print_messages(msg: Message) {
    let prefix = msg.prefix.unwrap_or_else(|| String::new());
    let command = msg.command;

    match command {
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

pub fn get_input(prompt: &str) -> String {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", TEXT);
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
    println!("{}", TEXT);
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
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
