use crate::application::dtos::{AccountDto, CreateAccountDto, UpdateAccountDto};
use crate::domain::entities::Account;
use crate::domain::repositories::AccountRepository;
use std::sync::Arc;

pub struct AccountService {
    repository: Arc<dyn AccountRepository>,
}

impl AccountService {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_account(&self, dto: CreateAccountDto) -> Result<AccountDto, String> {
        let account = Account::new(0, dto.username.clone());

        self.repository
            .save(&account)
            .await
            .map_err(|e| format!("Failed to save account: {:?}", e))?;

        Ok(AccountDto {
            id: account.id(),
            username: account.username().to_string(),
        })
    }

    pub async fn get_account(&self, id: i32) -> Result<Option<AccountDto>, String> {
        let account = self
            .repository
            .find_by_id(id)
            .await
            .map_err(|e| format!("Failed to find account: {:?}", e))?;

        Ok(account.map(|a| AccountDto {
            id: a.id(),
            username: a.username().to_string(),
        }))
    }

    pub async fn update_account(&self, id: i32, dto: UpdateAccountDto) -> Result<AccountDto, String> {
        let mut account = self
            .repository
            .find_by_id(id)
            .await
            .map_err(|e| format!("Failed to find account: {:?}", e))?
            .ok_or("Account not found")?;

        account.update_username(dto.username.clone());

        self.repository
            .save(&account)
            .await
            .map_err(|e| format!("Failed to update account: {:?}", e))?;

        Ok(AccountDto {
            id: account.id(),
            username: account.username().to_string(),
        })
    }

    pub async fn delete_account(&self, id: i32) -> Result<(), String> {
        self.repository
            .delete(id)
            .await
            .map_err(|e| format!("Failed to delete account: {:?}", e))?;

        Ok(())
    }
}
