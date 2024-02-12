use irc::{config::IrcConfig, users::{User, UserFlags}, IrcConnection};
use tokio::net::lookup_host;

use crate::utils::{get_input, print_messages};

pub async fn create_config() -> IrcConfig {
    let nickname = get_input("Enter your desired NICK name:");

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
