use std::time::Duration;

use irc::{
    self,
    messages::{Message, Params},
    IrcConfig, IrcConnection,
};
use tokio::{
    self,
    net::lookup_host,
    time::{sleep, sleep_until, Sleep},
};

#[tokio::main]
async fn main() {
    let (usr_nick, usr_name, host_name, server_name, real_name, address, port, channel) =
        create_user();

    connection = init_irc(
        usr_nick,
        usr_name,
        host_name,
        server_name,
        real_name,
        address,
        port,
    );

    irc_clien(connection);
}

pub fn create_user() -> (String, String, String, String, String, String, i32, String) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your desired NICK name:");
    let mut usr_nick = String::new();
    std::io::stdin().read_line(&mut usr_nick).unwrap();
    let usr_nick = usr_nick.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your desired username:");
    let mut usr_name = String::new();
    std::io::stdin().read_line(&mut usr_name).unwrap();
    let usr_name = usr_name.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your hostname:");
    let mut host_name = String::new();
    std::io::stdin().read_line(&mut host_name);
    let host_name = host_name.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your desired server name:");
    let mut server_name = String::new();
    std::io::stdin().read_line(&mut server_name).unwrap();
    let server_name = server_name.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your real name:");
    let mut real_name = String::new();
    std::io::stdin().read_line(&mut real_name).unwrap();
    let real_name = real_name.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter the server address");
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    let address = address.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter port:");
    let mut port = String::new();
    std::io::stdin().read_line(&mut port).unwrap();
    let port: i32 = port.trim().parse().unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter channel name:");
    let mut channel = String::new();
    std::io::stdin().read_line(&mut channel).unwrap();
    let channel = channel.trim().to_string();

    (
        usr_nick,
        usr_name,
        host_name,
        server_name,
        real_name,
        address,
        port,
        channel,
    )
}

async fn init_irc(
    usr_nick: String,
    usr_name: String,
    host_name: String,
    server_name: String,
    real_name: String,
    address: String,
    port: i32,
    //channel: String,
) -> irc::IrcConnection {
    let mut config = IrcConfig::new();
    let config = config
        .host(
            lookup_host(format!("{}:{}", address, port))
                .await
                .unwrap()
                .next()
                .unwrap(),
        )
        .set_receive_handler(|msg| {
            println!("{}", msg);
        });
    config.nickname = usr_nick;
    config.username = usr_name;
    config.hostname = host_name;
    config.servername = server_name;
    config.realname = real_name;

    let connection = config.connect().await.unwrap();

    connection
}

async fn irc_client(channel: String) {
    sleep(Duration::from_secs(3)).await;
    connection
        .send(Message {
            prefix: None,
            command: "PRIVMSG".to_string(),
            params: Params(vec![
                format!("#{} :", channel).to_string(),
                "another testing message".to_string(),
            ]),
        })
        .await
        .unwrap();
    sleep(Duration::from_secs(1)).await;
    connection
        .send(Message {
            prefix: None,
            command: "QUIT".to_string(),
            params: Params(vec![]),
        })
        .await
        .unwrap();
}

async fn send_message(connection: &mut IrcConnection, server_name: String) {
    println!("start your messages with 'msg' and your commands with 'cmd'");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() {
            //connection.send_raw(message).await.unwrap();
            if input.starts_with("msg") {
                connection
                    .send_raw(format!("PRIVMSG #test :{}", input))
                    .await
                    .unwrap();
            } else if input.starts_with("cmd") {
                connection.send_raw(format!("{}\n", input)).await.unwrap();
                break;
            }
        }
    }
}
