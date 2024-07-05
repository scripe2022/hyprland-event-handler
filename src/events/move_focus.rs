use std::sync::Arc;
use crate::global_state::GlobalState;
use serde_json::Value;
use crate::hyprsocket::Hyprsocket;
use std::error::Error;

pub struct MoveFocus;

impl MoveFocus {
    pub async fn handle(args: &[String], _state: Arc<GlobalState>, client: Arc<Hyprsocket>) -> Result<String, Box<dyn Error>> {
        if args.len() != 1 {
            return Err("move_focus requires exactly 1 argument".into());
        }
        let x = match client.send("j/activewindow").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e.into());
            },
        };
        let json: Value = serde_json::from_str(&x)?;
        let address = json["address"].as_str().unwrap_or("Unknown address");
        let class = json["class"].as_str().unwrap_or("Unknown class");

        let (hyprdir, kittydir) = match args[0].as_str() {
            "left" => ("l", "H"),
            "right" => ("r", "L"),
            "up" => ("u", "K"),
            "down" => ("d", "J"),
            _ => return Ok("invalid direction".to_string()),
        };

        if class == "kitty" {
            match client.sends_silent(&[format!("dispatch sendshortcut CONTROL, {}, address:{}", kittydir, address).as_str()]).await {
                Ok(_) => return Ok(format!("move_focus {} for window {}", hyprdir, address)),
                Err(e) => return Err(e.into()),
            }
        }
        else {
            match client.sends_silent(&[format!("dispatch movefocus {}", hyprdir).as_str()]).await {
                Ok(_) => return Ok(format!("move_focus {} for window {}", hyprdir, address)),
                Err(e) => return Err(e.into()),
            }
        }
    }
}

