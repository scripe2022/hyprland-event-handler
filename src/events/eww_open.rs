use crate::helper;
use std::sync::Arc;
use crate::global_state::GlobalState;
// use serde_json::Value;
use crate::hyprsocket::Hyprsocket;
use std::io::Cursor;
use serde::Serialize;
use csv::ReaderBuilder;
use std::error::Error;

pub struct EwwOpen;

impl EwwOpen {
    pub async fn handle(args: &[String], state: Arc<GlobalState>, client: Arc<Hyprsocket>) -> Result<String, Box<dyn Error>> {
        if args.len() != 0 {
            return Err("eww_todo_getall requires exactly 0 argument".into());
        }

        let todoist_inbox_orig = {
            let inbox = state.todoist_inbox.lock().await;
            inbox.clone()
        };
        let json_orig = serde_json::to_string(&*todoist_inbox_orig)?;

        let _ = helper::runcmd("todoist-cli", &["sync"]).await?;
        let todo_csv = helper::runcmd("todoist-cli", &["--csv", "list"]).await?;
        let json_new = helper::parse_inbox(todo_csv.as_str(), state.clone(), true).await?;
        if json_orig != json_new {
            helper::runcmd_async("eww", &["update", format!("TODO={}", json_new).as_str()]);
        }

        return Ok("".to_string());
    }
}

