// run  := cargo run
// dir  := .
// kid  :=

use std::error::Error;
use std::process::Command;
use std::sync::Arc;

use serde_json::Value;

use crate::global_state::GlobalState;
use crate::hyprsocket::Hyprsocket;

pub struct MoveSublimeText;

impl MoveSublimeText {
    pub async fn handle(
        args: &[String], _state: Arc<GlobalState>, client: Arc<Hyprsocket>
    ) -> Result<String, Box<dyn Error>> {
        if !args.is_empty() {
            return Err("switch_master_side requires exactly 1 argument".into());
        }

        let x_clients = match client.send("j/clients").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e);
            }
        };
        let json_clients: Value = serde_json::from_str(&x_clients)?;

        let x_aw = match client.send("j/activeworkspace").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e);
            }
        };
        let json_aw: Value = serde_json::from_str(&x_aw)?;
        let current_wid = json_aw["id"].as_i64().unwrap_or(1);

        let mut sublime_wid: Option<i64> = None;

        if let Value::Array(clients) = json_clients {
            for client in clients {
                if client["class"].as_str() == Some("sublime_text") {
                    let ws_id = client["workspace"]["id"].as_i64();
                    sublime_wid = ws_id;
                }
            }
        }
        else {
            eprintln!("JSON data is not an array");
            return Err("JSON data is not an array".into());
        }

        if let Some(sid) = sublime_wid {
            if current_wid == sid {
                eprintln!("sublime text already in current workspace");
                return Err("sublime text already in current workspace".into());
            }
            else {
                match client
                    .sends_silent(&[format!("dispatch movetoworkspace {},class:sublime_text", current_wid).as_str()])
                    .await
                {
                    Ok(_) => return Ok("move_sublime_text".to_string()),
                    Err(e) => return Err(e)
                }
            }
        }
        let _child = Command::new("subl").spawn().expect("failed to spawn process");
        Ok("sublime text not found, start sublime text".to_string())
    }
}
