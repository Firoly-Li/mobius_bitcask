use serde::{Deserialize, Serialize};

mod event;

mod service;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendInfo {
    pub active_name: String,
    pub key: String,
    pub time: u64,
    pub offset: u64,
    pub length: u64,
}

impl AppendInfo {
    pub fn new(active_name: &str, key: &str, time: u64, offset: u64, length: u64) -> Self {
        Self {
            active_name: active_name.to_string(),
            key: key.to_string(),
            time,
            offset,
            length,
        }
    }
}
