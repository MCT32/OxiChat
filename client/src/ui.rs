use std::io::{Write, stdout};
use crossterm::{QueueableCommand, queue, cursor};
use crossterm::style::Color;

pub struct Terminal {
    // struct Terminal fields
    size: (u16, u16),
    foreground_color: Color,
    background_color: Color,
}

impl Terminal {
    // constructor. still dont fully understand how this works but here we are.
    pub fn new() -> Self {
        Terminal { // initializes
            size: (0, 0),
            foreground_color: Color::Black,
            background_color: Color::White, 
        }
    }
    pub fn size(&self) -> (u16, u16) { // gets terminal size, i think?
        self.size
    }

    pub fn set_back_color(&mut self, color: Color) { // function to set background color (background)
        self.background_color = color;
    }
    pub fn set_front_color(&mut self, color: Color) { // function to set foreground color (text)
        self.foreground_color = color;
    }
}

pub fn draw_member_block(terminal: &Terminal) {

    let (cols, rows) = terminal.size(); // cop terminal sizes

    let members_width = (cols as f32 * 0.1) as u16; // this finds the height and width of the members_block
    let members_height = (rows as f32 * 0.1) as u16;

    let chat_width = cols - members_width;  // ✅
    let chat_height = rows;
    
    let member_block_x = members_width - chat_width;
    let mut member_block_y = 1;

    let mut stdout = stdout();

    stdout.queue(cursor::MoveTo(member_block_x, member_block_y));
    


    stdout.flush();
    
}
