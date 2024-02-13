use std::io::{Write, stdout};
use crossterm::{QueueableCommand, queue, cursor, terminal};
use crossterm::style::Color;

pub struct Terminal {
    size: (u16, u16),
    foreground_color: Color,
    background_color: Color,
}

impl Terminal {
    pub fn new() -> Self {
        let size = terminal::size().expect("Failed to get terminal size");
        Terminal {
            size,
            foreground_color: Color::Black,
            background_color: Color::White,
        }
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn set_back_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_front_color(&mut self, color: Color) {
        self.foreground_color = color;
    }
}

pub fn draw_member_block(terminal: &Terminal) {
    let (cols, rows) = terminal.size();

    let members_width = (rows as f32 * 0.1) as u16;
    let members_height = (cols) as u16;

    let chat_width = rows - members_width;
    let chat_height = cols;

    let member_block_x = chat_width - members_width;
    let member_block_y = 1;

    let mut stdout = stdout();

    for y in member_block_y..cols {
        stdout.queue(cursor::MoveTo(member_block_x, y)).unwrap(); // uh
        
        for x in member_block_x..rows {
            stdout.queue(crossterm::style::SetBackgroundColor(Color::Black)).unwrap();
            stdout.queue(crossterm::style::SetForegroundColor(Color::Blue)).unwrap();
            stdout.queue(crossterm::style::Print(" ")).unwrap(); // this doesnt work
        } 
    }

    stdout.flush().unwrap();
}
