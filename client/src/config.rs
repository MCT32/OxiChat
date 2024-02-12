use irc::{config::IrcConfig, users::{User, UserFlags}};
use tokio::net::lookup_host;
use crate::utils::{get_input, on_message_received, print_ascii_art};

pub async fn create_config() -> IrcConfig {
    print_ascii_art();
    
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
        .set_receive_handler(on_message_received)
        .host(lookup_host(format!("{}:{}", address, port)).await.unwrap().next().unwrap()).unwrap()
}
