use std::io;

use std::time::Duration;
use irc::{IrcConnection, messages::{Message, Params}};
use tokio::time::sleep;

const TEXT: &str = include_str!("./ascii.txt");

pub fn print_messages(msg: Message) {
    let prefix = msg.prefix.unwrap_or_else(|| String::new());
    let command = msg.command;
    let params = msg.params;

    match command.as_str() {
        "JOIN" => {
            println!("{} joined channel {}", prefix, params);
        }
        "NOTICE" => {
            let msg = &params.0.last().unwrap()[1..];
            println!("Notice from {}: {}", prefix, msg);
        }
        _ => {
            println!("{} {} {}", prefix, command, params);
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
            command: "JOIN".to_string(),
            params: Params(vec![
                format!("#{}", channel).to_string(),
            ]),
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
                command: "PRIVMSG".to_string(),
                params: Params(vec![
                    format!("#{}", channel).to_string(),
                    format!(":{}", input.to_string())
                ]),
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
