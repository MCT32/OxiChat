use crossterm::{cursor::MoveTo, event::{self, poll, read, Event, EventStream, KeyCode, KeyModifiers}};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::QueueableCommand;

use std::{io::Write, sync::{Arc, RwLock}};

pub struct Rectangle {
    pub x: usize, 
    pub y: usize,
    pub w: usize,
    pub h: usize
}

#[derive(Clone)]
pub struct Screen<W: Write> {
    pub stdout: W,
    pub h: u16,
    pub w: u16,
    pub chats: Arc<RwLock<Vec<String>>>,
    pub prompt: String,
    pub bar_char: String,
    pub bar: String
}

impl <W: Write> Screen<W> {
    pub fn chat_window(/*client_screen: &mut Screen<W>*/self, boundary: Rectangle) {

        let n = self.chats.len();
        let m = n.checked_sub(boundary.h).unwrap_or(0);
    
        for (dy, line) in self.chats.iter().skip(m).enumerate() {
            self.stdout.queue(MoveTo(boundary.x as u16, (boundary.y + dy) as u16 )).unwrap();
            let bytes = line.as_bytes();
            self.stdout.write(bytes.get(0..boundary.w).unwrap_or(bytes)).unwrap();
        }
    }
    pub fn render_all(mut self /*mut client_screen: &mut Screen<W>*/) {
        let chat_lock = &self.chats.read().unwrap();
        self.stdout.queue(Clear(ClearType::All)).unwrap();

        let chat_vec: Vec<String> = chat_lock.clone().to_vec();

        Screen::chat_window(self, Rectangle {
        x: 0,
        y: 0,
        w: self.w as usize,
        h: self.h as usize,
        });

        self.stdout.queue(MoveTo(0, self.h)).unwrap();
        self.stdout.write(self.bar.as_bytes()).unwrap();

        self.stdout.queue(MoveTo(0, self.h - 1)).unwrap();

        {
            let bytes = self.prompt.as_bytes();
            self.stdout.write(bytes.get(0..self.w as usize).unwrap_or(bytes)).unwrap();
        }

        self.stdout.flush().unwrap();
        }
}