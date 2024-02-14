// use std::io::{self, stdout, Read, Write, ErrorKind};
// use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
// use crossterm::cursor::{MoveTo};
// use crossterm::style::{Print, SetBackgroundColor, SetForegroundColor, Color};
// use crossterm::{execute, QueueableCommand};
// use crossterm::event::{read, poll, Event, KeyCode, KeyModifiers, KeyEventKind};

// // THE COLUMNS ARE THE ONES THAT GO UP AND FUCKING DOWN.
// // THE ROWS IS THE HEIGHT

// pub fn get_terminal_size() -> (u16, u16) { // this is used by main() to grab term size at the start. 
//     let (cols, rows) = crossterm::terminal::size();
//     Ok((cols, rows))
// }
