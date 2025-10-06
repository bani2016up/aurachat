use serde::{Deserialize, Serialize};
use schemars::JsonSchema;


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ChatMessage {
    pub id: usize,
    pub user: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SystemStats {
    pub connected_users: usize,
    pub total_messages: usize,
    pub memory_usage: String,
}


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TelegramChat {
    telegram_id: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StreammerMessage {
    text: String,
    username: String,
    chat: TelegramChat,
}
