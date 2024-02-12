mod config;
mod utils;

use std::{io::{self, Write}, sync::RwLock};
use std::sync::Mutex;
use crossterm::{execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};
use irc::messages::Message;
use std::sync::{Arc};
use tokio::sync::Mutex as TokioMutex;

const TEXT: &str = include_str!("./ascii.txt");

pub struct State {
    messages: Vec<Message>
}

pub static STATE: RwLock<State> = RwLock::new(State{
    messages: Vec::new()
});

#[tokio::main]
async fn main() {
    let config = config::create_config().await;

    print_ascii_art();
    println!("Enter channel name: ");
    let mut channel = String::new();

    io::stdin().read_line(&mut channel).unwrap();
    let channel = channel.trim().to_string();

    let mut connection = config.connect().await.unwrap();

    let state_arc = Arc::new(Mutex::new(State {
        messages: Vec::new(),
    })); // thanks chatgpt. i dont know what this does but it doesnt throw errors anymore

    utils::irc_client(&mut connection, channel.clone()).await;

    utils::send_message(Arc::new(TokioMutex::new(connection)), channel, state_arc).await;
}

fn print_ascii_art() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();

    let mut stdout = io::stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!("{} \n", TEXT)),
        ResetColor,
    ).unwrap();
    stdout.flush().unwrap();
}
