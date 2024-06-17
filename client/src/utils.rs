use crossterm::{
    execute, queue,
    terminal::{self, enable_raw_mode, Clear, EnterAlternateScreen},
};
use std::io::stdout;

pub struct Canvas {
    w: u16,
    h: u16,
    chat: Chat,
    newline_index: u16,
}

pub struct Chat {
    messages: Vec<Message>,
    context: (u32, u32),
}

pub struct Message {
    pub timestamp: String,
    pub content: String,
    pub author: Author,
    pub pos: u32,
}

pub struct Author {
    pub text: String,
    pub color: Color,
    pub styled: String,
}

pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Canvas {
    pub fn init_canvas() -> Result<(std::io::Stdout, Self), Box<dyn std::error::Error>> {
        let mut stdout: std::io::Stdout = stdout();
        execute!(
            stdout,
            EnterAlternateScreen,
            Clear(crossterm::terminal::ClearType::All)
        )?;
        let (w, h) = terminal::size().expect("Failed to get terminal size...");
        Ok((stdout, Self {}))
    }
}

impl Chat {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            messages: Vec::new(),
            context: (0, 0),
        })
    }
}

impl Message {
    pub fn
}
