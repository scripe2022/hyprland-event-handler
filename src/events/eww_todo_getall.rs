use crate::helper;
use std::sync::Arc;
use crate::global_state::GlobalState;
use crate::hyprsocket::Hyprsocket;
use std::error::Error;

pub struct EwwTodoGetall;

impl EwwTodoGetall {
    pub async fn handle(args: &[String], state: Arc<GlobalState>, _client: Arc<Hyprsocket>) -> Result<String, Box<dyn Error>> {
        if args.len() != 0 {
            return Err("eww_todo_getall requires exactly 0 argument".into());
        }

        let todoist_inbox = state.todoist_inbox.lock().await;
        let json = serde_json::to_string(&*todoist_inbox)?;

        return Ok(json);
    }
}

