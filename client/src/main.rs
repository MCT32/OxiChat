use std::time::Duration;

use irc::{
    self,
    messages::{Message, Params},
    IrcConfig, IrcConnection,
};
use tokio::{
    self,
    net::lookup_host,
    time::sleep,
};

#[tokio::main]
async fn main() {
    let config = create_config();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter channel name:");
    let mut channel = String::new();
    std::io::stdin().read_line(&mut channel).unwrap();
    let channel = channel.trim().to_string();

    let mut connection = config.await.connect().await.unwrap();

    irc_client(&mut connection, channel).await;

}

pub async fn create_config() -> IrcConfig {
    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your desired NICK name:");
    let mut nickname = String::new();
    std::io::stdin().read_line(&mut nickname).unwrap();
    let nickname = nickname.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your desired username:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let username = username.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your hostname:");
    let mut hostname = String::new();
    std::io::stdin().read_line(&mut hostname).unwrap();
    let hostname = hostname.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your desired server name:");
    let mut servername = String::new();
    std::io::stdin().read_line(&mut servername).unwrap();
    let servername = servername.trim().to_string();

    print!("\x1B[2J\x1B[1;1H");
    println!("Enter your real name:");
    let mut realname = String::new();
    std::io::stdin().read_line(&mut realname).unwrap();
    let realname = realname.trim().to_string();

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

    IrcConfig{
        host: lookup_host(format!("{}:{}", address, port)).await.unwrap().next().unwrap(),
        nickname,
        username,
        hostname,
        servername,
        realname,
        
        password: None,
        raw_receive_handler: None,
        receive_handler: Some(|msg| {
            println!("{}", msg);
        })

    }

}

async fn irc_client(connection: &mut IrcConnection, channel: String) {
    sleep(Duration::from_secs(10)).await;
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
    sleep(Duration::from_secs(3)).await;
    
}

async fn send_message(connection: &mut IrcConnection) {
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
            }
        }
    }
}