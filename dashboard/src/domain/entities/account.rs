
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Account {
    id: i32,
    username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Account {
    pub fn new(id: i32, username: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            username,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn update_username(&mut self, username: String) {
        self.username = username;
        self.updated_at = Utc::now();
    }
}
