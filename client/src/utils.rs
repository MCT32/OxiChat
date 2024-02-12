use std::io;
use std::io::Write;
use std::time::Duration;
use irc::{IrcConnection, messages::{Message, Command}};
use tokio::time::sleep;
use crossterm::{execute, terminal, cursor, style::{Print, ResetColor, SetForegroundColor, Color}};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;

use crate::State;


// dont forget you edited line 33 of lib.rs

use crate::STATE;

const TEXT: &str = include_str!("./ascii.txt");

pub fn on_message_received(message: Message) {

    let mut state = STATE.write().unwrap();

    state.messages.push(message);

    print_messages(&state.messages);

}

pub fn print_messages(messages: &Vec<Message>) {
    print_ascii_art();

    for message in messages {
        let prefix = match message.prefix.as_ref() {
            Some(message) => message,
            None => ""
        };

        match &message.command {

          /*Command::Pass() => {

            }

            Command::User() => {

            }


*/
            Command::Notice(receiver, msg) => {
                execute!(
                    io::stdout(),
                    SetForegroundColor(Color::Yellow),
                    Print(format!("Notice from {}: {}", receiver, msg)),
                    ResetColor,
                    Print("\n")
                ).unwrap();
            }
            Command::Join(channel) => {
                execute!(
                    io::stdout(),
                    SetForegroundColor(Color::Green),
                    Print(format!("{} joined channel {}", prefix, channel)),
                    ResetColor,
                    Print("\n")
                ).unwrap();
            }
            Command::Nick(nickname) => {
                execute!(
                    io::stdout(),
                    SetForegroundColor(Color::Blue),
                    Print(format!("{} is your nickname.", &nickname)),
                    ResetColor,
                    Print("\n"),
                ).unwrap();

            }
            command => {
                if let Command::Raw(command, params) = command.raw() {
                    execute!(
                        io::stdout(),
                        SetForegroundColor(Color::Cyan),
                        Print(format!("{} {} {}", prefix, command, params.join(" "))),
                        ResetColor,
                        Print("\n")
                    ).unwrap();
                }
            }
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    print!("\x1B[2J\x1B[1;1H");
    print_ascii_art();
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub async fn irc_client(connection: &mut IrcConnection, channel: String) {
    sleep(Duration::from_secs(2)).await;
    connection
        .send(Message {
            prefix: None,
            command: Command::Join(format!("#{}", channel).to_string()),
        })
        .await
        .unwrap();
    sleep(Duration::from_secs(2)).await;
}

pub async fn send_message(connection: Arc<TokioMutex<IrcConnection>>, channel: String, state: Arc<Mutex<State>>) { // this is fucking fuckity fucked up but it works
    print_ascii_art(); 
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if !input.is_empty() {
            let mut conn = connection.lock().await;
            conn.send(Message {
                prefix: None,
                command: Command::PrivMsg(format!("#{}", channel), input.clone()),
            }).await.unwrap();

            if input == "QUIT" {
                conn.quit().await.unwrap();
                break;
            }

            let mut state = state.lock().unwrap();
            state.messages.push(Message {
                prefix: Some("You".to_string()),
                command: Command::PrivMsg(format!("#{}", channel), input),
            });
            print_messages(&state.messages);
        }
    }
}

pub fn print_ascii_art() {
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
