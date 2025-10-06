use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AccountDto {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateAccountDto {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateAccountDto {
    pub username: String,
}
