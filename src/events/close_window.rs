use std::sync::Arc;
use crate::global_state::GlobalState;
use serde_json::Value;
use crate::hyprsocket::Hyprsocket;
use std::error::Error;

pub struct CloseWindow;

impl CloseWindow {
    pub async fn handle(args: &[String], _state: Arc<GlobalState>, client: Arc<Hyprsocket>) -> Result<String, Box<dyn Error>> {
        if !args.is_empty() {
            return Err("close_window requires exactly 0 argument".into());
        }
        let x = match client.send("j/activewindow").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e);
            },
        };
        let json: Value = serde_json::from_str(&x)?;
        let address = json["address"].as_str().unwrap_or("Unknown address");
        let class = json["class"].as_str().unwrap_or("Unknown class");

        if class == "kitty" || class == "firefox" || class == "sublime_text" {
            match client.sends_silent(&[format!("dispatch sendshortcut CONTROL, W, address:{}", address).as_str()]).await {
                Ok(_) => Ok(format!("close_window for window {}", address)),
                Err(e) => Err(e),
            }
        }
        else if class == "com.obsproject.Studio" {
            return Ok("do nothing for obs studio".to_string());
        }
        else {
            match client.sends_silent(&["dispatch killactive"]).await {
                Ok(_) => Ok(format!("killactive for window {}", address)),
                Err(e) => Err(e),
            }
        }
    }
}

