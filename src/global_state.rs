use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct GlobalState {
    pub scratchpad_address: Arc<Mutex<HashMap<String, String>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            scratchpad_address: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

