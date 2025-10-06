
use crate::domain::entities::Account;
use crate::domain::repositories::{AccountRepository, RepositoryError, Result};
use crate::infrastructure::persistence::models::account::{self, Entity as AccountEntity};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait};

pub struct AccountRepositoryImpl {
    db: DatabaseConnection,
}

impl AccountRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn model_to_entity(model: account::Model) -> Account {
        Account::new(model.id, model.username)
    }

    fn entity_to_active_model(account: &Account) -> account::ActiveModel {
        account::ActiveModel {
            id: Set(account.id()),
            username: Set(account.username().to_string()),
            created_at: Set(account.created_at().naive_utc()),
            updated_at: Set(account.updated_at().naive_utc()),
        }
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn find_by_id(&self, id: i32) -> Result<Option<Account>> {
        let account = AccountEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(account.map(Self::model_to_entity))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<Account>> {
        let account = AccountEntity::find()
            .filter(account::Column::Username.eq(username))
            .one(&self.db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(account.map(Self::model_to_entity))
    }

    async fn find_all(&self) -> Result<Vec<Account>> {
        let accounts = AccountEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(accounts.into_iter().map(Self::model_to_entity).collect())
    }

    async fn save(&self, account: &Account) -> Result<()> {
        let active_model = Self::entity_to_active_model(account);
        
        active_model
            .insert(&self.db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<()> {
        AccountEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
