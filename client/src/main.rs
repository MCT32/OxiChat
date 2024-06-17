mod client;
mod error;
mod utils;

use crate::utils::Canvas;
use irc::*;

use std::io::{stdout, Stdout};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (ping_1, mut pong_1): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(10);
    // [ ping_1 = Frontend TX, pong_1 = Backend RX ] -> Frontend to backend comms
    let (ping_2, mut pong_2): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(10);
    // [ ping_2 = Backend TX, pong_2 = Frontend RX ] -> Backend to frontend comms

    let (mut stdout, mut canvas): (Stdout, Canvas) = Canvas::init_canvas().unwrap();

    canvas.leave_canvas(stdout).unwrap();
}
