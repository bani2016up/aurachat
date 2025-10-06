use crate::domain::entities::Account;
use async_trait::async_trait;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<Account>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<Account>>;
    async fn find_all(&self) -> Result<Vec<Account>>;
    async fn save(&self, account: &Account) -> Result<()>;
    async fn delete(&self, id: i32) -> Result<()>;
}
