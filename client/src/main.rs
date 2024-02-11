use std::{io, time::Duration};

use irc::{
    self, config::IrcConfig, messages::{Message, Params}, users::{User, UserFlags}, IrcConnection
};
use tokio::{
    self,
    net::lookup_host,
    time::sleep,
};

#[tokio::main]
async fn main() {
    let config = create_config().await;

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter channel name:");
    let mut channel = String::new();
    std::io::stdin().read_line(&mut channel).unwrap();
    let channel = channel.trim().to_string();

    let mut connection = config.connect().await.unwrap();

    irc_client(&mut connection, channel.clone()).await;

    send_message(&mut connection, channel).await;

}

async fn create_config() -> IrcConfig {
    let nickname = get_input("Enter your desired NICK name:"); /* 
    let username = get_input("Enter your desired username:");
    let hostname = get_input("Enter your hostname:");
    let servername = get_input("Enter your desired server name:");
    let realname = get_input("Enter your real name:"); */

    let username = nickname.clone();
    let hostname = nickname.clone();
    let servername = nickname.clone();
    let realname = nickname.clone(); // TODO: all of this will come back after the tui...exists. 

    let address = get_input("Enter the server address:");
    let port: u16 = get_input("Enter port:").parse().expect("Invalid port");

    IrcConfig::builder()
        .user(User{
            nickname,
            username,
            hostname,
            servername,
            realname,
            flags: UserFlags::default(),
        })
        //.password
        .set_receive_handler(print_messages)
        .host(lookup_host(format!("{}:{}", address, port)).await.unwrap().next().unwrap()).unwrap()
}

fn print_messages(msg: Message) {
    let prefix = msg.prefix.unwrap_or_else(|| String::new()); 
    let command = msg.command;
    let params = msg.params.0.join(" ");

    println!("{:?}: {} {}", prefix, command, params);
}

fn get_input(prompt: &str) -> String {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

async fn irc_client(connection: &mut IrcConnection, channel: String) {
    sleep(Duration::from_secs(5)).await;
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

async fn send_message(connection: &mut IrcConnection, channel: String) {
    println!("you're in the chat. | go ham. ");
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
            } 
        }
    }
}