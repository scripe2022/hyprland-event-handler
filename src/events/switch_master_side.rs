// run  := cargo run
// dir  := .
// kid  := 

use std::sync::Arc;
use crate::global_state::GlobalState;
use serde_json::Value;
use crate::hyprsocket::Hyprsocket;
use std::error::Error;

pub struct SwitchMasterSide;

impl SwitchMasterSide {
    pub async fn handle(args: &[String], _state: Arc<GlobalState>, client: Arc<Hyprsocket>) -> Result<String, Box<dyn Error>> {
        if !args.is_empty() {
            return Err("switch_master_side requires exactly 1 argument".into());
        }
        let x_cur = match client.send("j/activewindow").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e);
            },
        };
        let json_cur: Value = serde_json::from_str(&x_cur)?;
        let addr_cur = json_cur["address"].as_str().unwrap_or("Unknown address");

        match client.sends_silent(&["dispatch layoutmsg focusmaster"]).await {
            Ok(_) => {},
            Err(e) => return Err(e),
        }

        let x_master = match client.send("j/activewindow").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e);
            },
        };
        let json_master: Value = serde_json::from_str(&x_master)?;
        let addr_master = json_master["address"].as_str().unwrap_or("Unknown address");

        let mut direction = "";

        if addr_cur != addr_master {
            match client.sends_silent(&["dispatch movefocus l"]).await {
                Ok(_) => {},
                Err(e) => return Err(e),
            }

            let x_new = match client.send("j/activewindow").await {
                Ok(output) => output,
                Err(e) => {
                    eprintln!("Error sending command: {}", e);
                    return Err(e);
                },
            };
            let json_new: Value = serde_json::from_str(&x_new)?;
            let addr_new = json_new["address"].as_str().unwrap_or("Unknown address");

            if addr_cur == addr_new {
                direction = "r";
            }
            else {
                direction = "l";
                match client.sends_silent(&[format!("dispatch focuswindow address:{}", addr_cur).as_str()]).await {
                    Ok(_) => {},
                    Err(e) => return Err(e),
                }
            }
        }

        if !direction.is_empty() {
            match client.sends_silent(&[format!("dispatch swapwindow {}", direction).as_str()]).await {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
            match client.sends_silent(&[format!("dispatch swapwindow {}", direction).as_str()]).await {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
            match client.sends_silent(&[format!("dispatch focuswindow address:{}", addr_master).as_str()]).await {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
            match client.sends_silent(&["dispatch layoutmsg swapwithmaster"]).await {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
            match client.sends_silent(&[format!("dispatch focuswindow address:{}", addr_cur).as_str()]).await {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        Ok("switch_master_side".to_string())
    }
}

