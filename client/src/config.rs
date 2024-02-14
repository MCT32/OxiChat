use irc::{config::IrcConfig, users::{User, UserFlags}};
use tokio::net::lookup_host;

use crate::client_utils::{on_message_received};

pub async fn create_config(nickname: String, address: String, port: u16) -> IrcConfig {
    let nickname = nickname;

    let username = nickname.clone();
    let hostname = nickname.clone();
    let servername = nickname.clone();
    let realname = nickname.clone(); // TODO: all of this will come back after the tui...exists. EDIT nevermind lol fuck allat

    let address = address;
    let port: u16 = port;

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
        .set_receive_handler(on_message_received)
        .host(lookup_host(format!("{}:{}", address, port)).await.unwrap().next().unwrap()).unwrap()
}