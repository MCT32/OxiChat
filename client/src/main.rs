mod client;
mod utils;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (ping_1, mut pong_1): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(10);
    // [ ping_1 = Frontend TX, pong_1 = Backend RX ] -> Frontend to backend comms
    let (ping_2, mut pong_2): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(10);
    // [ ping_2 = Backend TX, pong_2 = Frontend RX ] -> Backend to frontend comms
}

pub async fn backend_task(tx: mpsc::Sender<String>, mut rx: mpsc::Receiver<String>) {
    while let Some(message) = rx.recv().await {
        println!("we got a message! it reads: {:?}", message);
        tx.send("monkey!".to_string())
            .await
            .expect("code failed, seppuku is only way of honors");
    }
}
