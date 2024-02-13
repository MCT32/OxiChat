mod config;
mod utils;
mod ui;

use std::{io::{self, Write}, sync::RwLock};
use crossterm::{execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};
use irc::messages::Message;

use crate::ui::Terminal;

const TEXT: &str = include_str!("./ascii.txt");

pub struct State {
    messages: Vec<Message>
}

pub static STATE: RwLock<State> = RwLock::new(State{
    messages: Vec::new()
});

#[tokio::main]
async fn main() {

    let mut terminal = Terminal::new();
    terminal.set_back_color(Color::Black);
    terminal.set_front_color(Color::White);
    let (term_width, term_height) = terminal.size();

    let config = config::create_config().await;

    print_ascii_art();
    print!("Enter channel name: ");
    io::stdout().flush().unwrap(); 
    let mut channel = String::new();
    io::stdin().read_line(&mut channel).expect("Failed to read input");


    let channel = channel.trim().to_string();

    let mut connection = config.connect().await.unwrap();

    utils::irc_client(&mut connection, channel.clone()).await;

    let nickname = config.user.nickname.clone();

    utils::send_message(&mut connection, channel, nickname).await;
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