use tokio::net::UnixStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader};
use std::error::Error;
use std::env;

pub struct Hyprsocket {
    command_socket_path: String,
}

impl Hyprsocket {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")?;
        let hyprland_instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;

        let command_socket_path = format!("{}/hypr/{}/.socket.sock", xdg_runtime_dir, hyprland_instance_signature);

        Ok(Self {
            command_socket_path,
        })
    }

    pub async fn send(&self, command: &str) -> Result<String, Box<dyn Error>> {
        let mut stream = UnixStream::connect(&self.command_socket_path).await?;
        stream.write_all(command.as_bytes()).await?;

        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_to_string(&mut response).await?;

        Ok(response)
    }

    pub async fn sends_silent(&self, commands: &[&str]) -> Result<(), Box<dyn Error>> {
        let mut stream = UnixStream::connect(&self.command_socket_path).await?;
        let command = if commands.len() > 1 {
            format!("[[BATCH]]{}", commands.join(";"))
        } else {
            commands.get(0).map_or(String::new(), |&cmd| cmd.to_string())
        };

        if !command.is_empty() {
            stream.write_all(command.as_bytes()).await?;
        }

        Ok(())
    }
}

