// use std::error::Error;

use core::time;
use std::thread;

use crate::utils::Canvas;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode},
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
    pub async fn mainloop(self, stdout: std::io::Stdout) {
        for _ in 0..=5 {
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
    }
    pub fn render(
        &mut self,
        mut stdout: &mut std::io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }
    pub fn event_loop() -> Result<(), Box<dyn std::error::Error>> {
        let mut quit: bool = false; // a genius admires simplicity
        while !quit {
            if poll(time::Duration::from_millis(66))? {
                if let Event::Key(key) = read()? {
                    match key.code {
                        KeyCode::Char(x) => {}
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
    pub fn lex(&mut self, key: &KeyEvent) {}
}
