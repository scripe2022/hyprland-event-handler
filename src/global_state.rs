use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Record {
    pub id: String,
    pub priority: String,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct GlobalState {
    pub scratchpad_address: Arc<Mutex<HashMap<String, String>>>,
    pub todoist_inbox: Arc<Mutex<Vec<Record>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            scratchpad_address: Arc::new(Mutex::new(HashMap::new())),
            todoist_inbox: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

