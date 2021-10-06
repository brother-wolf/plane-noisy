use chrono::{DateTime, Utc};
use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Tweet {
    pub id: u64,
    pub epoch: i64,
    pub date_time: String,
    pub content: String
}

impl Tweet {
    pub fn new(id: u64, date_time: DateTime::<Utc>, content: String) -> Tweet {
        Tweet { id, epoch: date_time.timestamp_millis(), date_time: date_time.format("%Y-%m-%d %H:%M:%S").to_string(), content}
    }
}
