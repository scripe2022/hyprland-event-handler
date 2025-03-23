use std::error::Error;
use std::sync::Arc;

use serde_json::Value;

use crate::global_state::GlobalState;
use crate::hyprsocket::Hyprsocket;

pub struct ToggleScratch;

impl ToggleScratch {
    pub async fn handle(
        args: &[String], state: Arc<GlobalState>, client: Arc<Hyprsocket>
    ) -> Result<String, Box<dyn Error>> {
        if !args.is_empty() {
            return Err("toggle_scratch requires exactly 0 argument".into());
        }
        let x = match client.send("j/activewindow").await {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error sending command: {}", e);
                return Err(e);
            }
        };
        let json: Value = serde_json::from_str(&x)?;
        let address = json["address"].as_str().unwrap_or("Unknown address");
        let workspace_name = json["workspace"]["name"].as_str().unwrap_or("Unknown workspace name");

        let mut map = state.scratchpad_address.lock().await;
        if workspace_name != "special:scratchpad" {
            map.insert(address.to_string(), workspace_name.to_string());
            match client.sends_silent(&["dispatch movetoworkspacesilent special:scratchpad"]).await {
                Ok(_) => Ok(format!("Move window {} from {} to special:scratchpad", address, workspace_name)),
                Err(e) => Err(e)
            }
        }
        else if let Some(workspace) = map.remove(address) {
            match client.sends_silent(&[format!("dispatch movetoworkspace {}", workspace).as_str()]).await {
                Ok(_) => return Ok(format!("Move window {} from special:scratchpad to {}", address, workspace)),
                Err(e) => return Err(e)
            }
        }
        else {
            return Ok("key does not exist".to_string());
        }
    }
}
