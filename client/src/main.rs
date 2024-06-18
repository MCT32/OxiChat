mod client;
mod error;
mod utils;

use crate::utils::{Arguments, Canvas};
use client::OxiChat;
use irc::irc_enums::{IrcCommand, IrcEvent};
use utils::ParseResult;

use std::{
    env::{self, args, Args},
    io::{stdout, Stdout},
};

use tokio::sync::mpsc::{self, channel, Receiver, Sender};

#[tokio::main]
async fn main() {
    let (ping_1, mut pong_1): (Sender<_>, Receiver<_>) = channel(32);
    // [ ping_1 = Frontend TX, pong_1 = Backend RX ] -> Frontend to backend comms
    let (ping_2, mut pong_2): (Sender<_>, Receiver<_>) = channel(32);
    // [ ping_2 = Backend TX, pong_2 = Frontend RX ] -> Backend to frontend comms

    let (mut stdout, mut canvas): (Stdout, Canvas) = Canvas::init_canvas().unwrap();
    let mut oxichat = OxiChat::new(canvas); // consumes canvas
    let args: Vec<String> = env::args().collect();

    let args = Arguments::parse_arguments(args);

    match args.unwrap() {
        ParseResult::Config(config) => {
            println!("configgered! {:?}", config);
            oxichat.construct(config)
        }
        ParseResult::Args(args) => {
            println!("arg'd! {:?}", args)
            // this should never happen, for now at least
        }
    }
    println!("{:?}", oxichat); // dbg
    let config = oxichat.config.clone();
    let stdout_a: Stdout = std::io::stdout();
    let stdout_b: Stdout = std::io::stdout();

    let frontend_handle = tokio::task::spawn(async move {
        oxichat.clone().mainloop(stdout_a).await;
        oxichat
            .canvas
            .leave_canvas(stdout_b)
            .expect("code failed must kms");
    });

    let backend_handle =
        tokio::task::spawn(async move { irc::backend(config.unwrap(), ping_2, pong_1).await });

    let (frontend_result, backend_result) =
        tokio::try_join!(frontend_handle, backend_handle).unwrap();
}
