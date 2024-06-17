use std::time::Duration;

use irc::{
    config::IrcConfig,
    users::{User, UserFlags},
};
use tokio::{net::lookup_host, time::sleep};

use crate::utils::{message_receiver, Client};

pub async fn create_irc_config(config: Client) -> IrcConfig {
    let nickname = config.nickname;

    let username = nickname.clone();
    let hostname = nickname.clone();
    let servername = nickname.clone();
    let realname = nickname.clone(); // TODO: all of this will come back after the tui...exists. EDIT nevermind lol fuck allat

    let address = config.address;
    let port: u16 = config.port;

    IrcConfig::builder()
        .user(User {
            nickname,
            username,
            hostname,
            servername,
            realname,
            flags: UserFlags::default(),
        })
        //.password
        .set_receive_handler(message_receiver)
        .host(
            lookup_host(format!("{}:{}", address, port))
                .await
                .unwrap()
                .next()
                .unwrap(),
        )
        .unwrap()
}
