use crossterm::{
    execute, queue,
    terminal::{self, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
};
use std::io::stdout;

#[derive(Clone, PartialEq, Debug)]
pub struct Canvas {
    pub w: u16,
    pub h: u16,
    pub chat: Chat,
    pub domain: Domain,
    pub newline_index: u16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Chat {
    pub messages: Vec<Message>,
    pub context: (u32, u32), // context (# - #) according to height of screen, also cannot display default message
}

#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    pub timestamp: String,
    pub content: String,
    pub author: Author,
    pub pos: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Author {
    pub text: String,
    pub color: Color,
    pub styled: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Domain {
    // chat domain
    pub a: (u32, u32), // top left point            a --------- b
    pub b: (u32, u32), // top right point             | area  |
    pub c: (u32, u32), // bottom right point          |       |
    pub d: (u32, u32), // bottom left point         d --------- c
}

#[derive(Clone, PartialEq, Debug)]
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
        enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            Clear(crossterm::terminal::ClearType::All)
        )?;
        let (w, h) = terminal::size().expect("Failed to get terminal size...");
        Ok((
            stdout,
            Self {
                w,
                h,
                chat: Chat::new(),
                domain: Domain::new(),
                newline_index: 0,
            },
        ))
    }
    pub fn leave_canvas(
        &mut self,
        mut stdout: std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        execute!(
            stdout,
            Clear(crossterm::terminal::ClearType::All),
            LeaveAlternateScreen
        )?;
        Ok(())
    }
}

impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            context: (0, 0),
        }
    }
}

impl Message {
    pub fn new() -> Self {
        // this returns a very default-looking message. it will be ignored by the renderer
        Self {
            timestamp: "%H/%M".to_owned(),
            content: "[ CONTENT ]".to_owned(),
            author: Author::new(),
            pos: -1, // this will be ignored
        }
    }
}

impl Author {
    pub fn new() -> Self {
        // very default looking author.
        Self {
            text: "[ AUTHOR ]".to_owned(),
            color: Color::Red,
            styled: "[TD]".to_owned(),
        }
    }
}

impl Domain {
    pub fn new() -> Self {
        // empty Domain
        Self {
            a: (0, 0),
            b: (10, 0),
            c: (10, 10),
            d: (0, 10),
            // 10x10 square
        }
    }
}

pub fn parse_arguments(args: Vec<String>) {
    todo!()
}
