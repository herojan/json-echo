use parking_lot::RwLock;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub json_value: Arc<RwLock<Value>>,
}

impl State {
    pub fn new() -> Self {
        State {
            json_value: Arc::new(RwLock::new(json!({"value": 0}))),
        }
    }
}
