use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct GlobalState {
    pub scratchpad_address: Arc<Mutex<HashMap<String, String>>>,
    pub current_magnifier_scale: Arc<Mutex<f64>>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            current_magnifier_scale: Arc::new(Mutex::new(1.0)),
            scratchpad_address: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

