mod config;
mod utils;

const TEXT: &str = include_str!("./ascii.txt");

#[tokio::main]
async fn main() {
    let config = config::create_config().await;

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", TEXT);
    println!("Enter channel name: ");
    let mut channel = String::new();
    std::io::stdin().read_line(&mut channel).unwrap();
    let channel = channel.trim().to_string();

    let mut connection = config.connect().await.unwrap();

    utils::irc_client(&mut connection, channel.clone()).await;

    utils::send_message(&mut connection, channel).await;
}