use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;

use std::{io::{stdout, Stdout, Write}, sync::{Arc, RwLock}};

use crate::{ChatsRef, Client, CHATS};

#[derive(Debug, Clone)]
pub struct Client_Screen {

    pub w: u16,
    pub h: u16,

    pub chats: Arc<RwLock<Vec<String>>>,
    pub input: String,

    pub bar_char: String,
    pub bar: String, 
}

impl Client_Screen {
    pub fn default_client_screen() -> Self {

        let (mut h, mut w) = terminal::size().expect("Fuck!");
        let mut chats = CHATS.read().unwrap();
        let bar_char = "█";

        Client_Screen  {
            w,
            h,

            chats: CHATS.clone(),

            bar_char: "█".to_owned(),
            bar: bar_char.repeat(w as usize), 

            input: "".to_string(),
        }
    }
}

pub fn renderer(client_screen: Client_Screen) {
    
}

pub fn initialize_screen() {
    let _ = crossterm::terminal::enable_raw_mode();
}

pub fn deinitialize_screen() {
    let _ = crossterm::terminal::disable_raw_mode();
}

pub fn render(mut stdout: &Stdout) {
    let chats = CHATS.read().unwrap().to_vec();

    stdout.queue(MoveTo(0, 1));
    let chats = chats;
    println!("{:?}", chats);
}