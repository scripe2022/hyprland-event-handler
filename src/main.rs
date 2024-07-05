mod events {
    pub mod toggle_scratch;
    pub mod close_window;
    pub mod move_focus;
}
mod global_state;
mod hyprsocket;

use hyprsocket::Hyprsocket;
use global_state::GlobalState;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::fs::remove_file;

use crate::events::toggle_scratch::ToggleScratch;
use crate::events::close_window::CloseWindow;
use crate::events::move_focus::MoveFocus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Arc::new(Hyprsocket::new().await?);


    let socket_path = "/tmp/event_handler_test.sock";

    let _ = remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)?;

    let state = Arc::new(GlobalState::new());

    loop {
        let (mut socket, _) = listener.accept().await?;
        let state = Arc::clone(&state);
        let client = Arc::clone(&client);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let n = match socket.read(&mut buf).await {
                Ok(n) => n,
                Err(_) => return,
            };
            let input = String::from_utf8(buf[..n].to_vec()).unwrap();

            let response = match handle_command(input, state, client).await {
                Ok(output) => output,
                Err(err) => format!("Error: {}", err),
            };

            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Failed to send response: {}", e);
            }
        });
    }
}

async fn handle_command(input: String, state: Arc<GlobalState>, client: Arc<Hyprsocket>) -> Result<String, Box<dyn Error>> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Err("No command provided".into());
    }

    let event = parts[0];
    let args = parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>();

    match event {
        "toggle_scratch" => ToggleScratch::handle(&args, state, client).await,
        "close_window" => CloseWindow::handle(&args, state, client).await,
        "move_focus" => MoveFocus::handle(&args, state, client).await,
        _ => Err(format!("Unknown event: {}", event).into()),
    }
}

