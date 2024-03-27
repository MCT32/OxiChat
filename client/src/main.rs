mod client_utils;
mod ui_utils;
mod config;
mod errors;

use client_utils::*;
use ui_utils::*;
use config::*;
use error::*;
use irc::*;

use irc::config::IrcConfig;

use errors::terminal_error;
use errors::network_error;
use errors::argument_error;

use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;

use lazy_static::lazy_static;

use std::{env, io::{self, stdin, stdout, Write}, process, string, sync::RwLock, thread, time::Duration};
use std::sync::{Mutex, Arc};
use std::process::exit;

use futures::{FutureExt, StreamExt};

// way too fucking many imports right now that will be fixed (TODO?)

const NERDROOM_ASCII: &str = include_str!("./ascii.txt"); // ascii art to be printed

pub type ChatsRef = Arc<RwLock<Vec<String>>>; // custom type for this so that i dont have to write out this fucking mess every time.

lazy_static!{
    pub static ref CHATS: ChatsRef = Arc::new(RwLock::new(Vec::new())); // sort of global chat vector
}

#[tokio::main]
async fn main() { // async main function. this probably has too much logic in it but thats okay. [TODO: Factor out some fucking code]

// initialize all variables {
    let mut stdout = stdout(); // initialize standard output
    let (mut w, mut h) = terminal::size().expect("
    Failed to get the fucking terminal size. Something is very very wrong. This should never happen.
    ");

    let args: Vec<String> = env::args().collect(); // collect startup arguments
    check_args_valid(&args); // check validity of startup arguments

    let mut reader = EventStream::new(); // initialize async eventstream for sucking events

    let mut client = Client::default_client_configuration(args); // instantiate client based on startup args
    let mut client_screen = Client_Screen::default_client_screen(); // instantiate client screen
    let mut irc_configuration: IrcConfig = config::create_irc_config(client.clone()).await;

    // let mut faculties = Faculties::create(client, client_screen, irc_configuration);

    // client::connect. need to implement this

// all variables initialized }

    loop { // main event loop starts here
        let event = reader.next().await.unwrap().unwrap();
        match event {
            Event::Resize(nw, nh) => {
                client_screen.w = nw;
                client_screen.h = nh;
            }

            Event::Paste(data) => {
                client_screen.input.push_str(&data);
            }

            Event::Key(event) => {
                key_handler(&mut client_screen, &mut client, &mut irc_configuration, event).await;
            }
            _ => {
                terminal_error();
            }
        }

    } // main event loop ends here.

} // async main funciton ends here. that should be pretty obvious though. 