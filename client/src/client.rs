// use std::error::Error;

use core::time;
use std::thread;

use crate::utils::Canvas;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{Clear, ClearType},
};
use irc::config::IrcConfig;

#[derive(Clone, PartialEq, Debug)]
pub struct OxiChat {
    pub canvas: Canvas,
    pub config: Option<IrcConfig>,
}

impl OxiChat {
    pub fn new(canvas: Canvas) -> Self {
        Self {
            canvas,
            config: None,
        }
    }
    pub fn construct(&mut self, config: IrcConfig) {
        self.config = Some(config);
    }
    // pub async fn mainloop(self, stdout: std::io::Stdout) {
    //     for _ in 0..=5 {
    //         tokio::time::sleep(time::Duration::from_secs(1)).await;
    //     }
    // }
    pub fn render(
        &mut self,
        mut stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }
    pub async fn mainloop(
        &mut self,
        mut stdout: std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut quit: bool = false; // a genius admires simplicity
        while !quit {
            if poll(time::Duration::from_millis(66))? {
                match read()? {
                    Event::Resize(nw, nh) => {
                        self.canvas.w = nw;
                        self.canvas.h = nh;
                    }
                    Event::Key(event) => self.lex(event),
                    _ => {
                        // catch all for match read()? {}
                    }
                }
            }
        }
        Ok(())
    }
    pub fn lex(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace => {
                self.canvas.input.pop();
            }
            KeyCode::Tab => self.canvas.input.push_str("     "), // tab character, 5 spaces
            KeyCode::Char(x) => {
                if x == 'c' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.clean_exit()
                } else {
                    self.canvas.input.push(x)
                }
            }
            KeyCode::Enter => {
                print!("he pressed enter!")
            }
            _ => {
                // lolz
            }
        }
    }
    pub fn clean_exit(&mut self) {
        let stdout = std::io::stdout();
        self.canvas.leave_canvas(stdout);
        std::process::exit(0);
    }
}
