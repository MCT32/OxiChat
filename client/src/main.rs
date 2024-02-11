use std::time::Duration;

use irc::{self, IrcConnection};
use tokio::{
    self,
    net::lookup_host,
    time::{sleep, sleep_until, Sleep},
};

#[tokio::main]
async fn main() {
    let mut connection = initialize_irc().await;
    send_message(&mut connection).await;
}

async fn initialize_irc() -> irc::IrcConnection {
    println!("Enter a nickname:");
    let mut usr_nick = String::new();
    std::io::stdin().read_line(&mut usr_nick).unwrap();

    println!("Enter a username:");
    let mut usr_name = String::new();
    std::io::stdin().read_line(&mut usr_name).unwrap();

    println!("Enter the server name (WITH THE #):");
    let mut server_name = String::new();
    std::io::stdin().read_line(&mut server_name).unwrap();

    let mut config = irc::IrcConfig::new();
    let config = config
        .host(
            lookup_host("irc.libera.chat:6667")
                .await
                .unwrap()
                .next()
                .unwrap(),
        )
        .receive_handler(|msg| {
            println!("{}", msg);
        });

    let mut connection = config.connect().await.unwrap();

    sleep(Duration::from_secs(5)).await;

    connection
        .send_raw(format!("NICK {}\n", usr_nick))
        .await
        .unwrap();

    let mut myasshole = String::new();
    let mut myasshole: &str = "banana";

    sleep(Duration::from_secs(1)).await;

    connection
        .send_raw(format!(
            "USER {} {} {} {}\n",
            usr_nick, usr_name, server_name, myasshole
        ))
        .await
        .unwrap();

    sleep(Duration::from_secs(3)).await;

    connection
        .send_raw(format!("JOIN {}\n", server_name))
        .await
        .unwrap();

    sleep(Duration::from_secs(1)).await;

    connection
}

async fn send_message(connection: &mut IrcConnection) {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() {
            let message = format!("PRIVMSG #test :{}\n", input);
            connection.send_raw(message).await.unwrap();
        } else if input == "QUIT" {
            connection.send_raw("QUIT\n").await.unwrap();
            break; // Break out of the loop if the user inputs QUIT
        }
    }
}
