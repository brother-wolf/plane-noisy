use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tweet {
    pub id: u64,
    pub date_time: DateTime::<Utc>,
    pub content: String
}

impl Tweet {
    pub fn new(id: u64, date_time: DateTime::<Utc>, content: String) -> Tweet {
        Tweet { id, date_time, content}
    }
}
