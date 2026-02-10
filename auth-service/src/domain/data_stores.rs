use super::{User, Email, Password};

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;

    async fn get_user(&self, email: &Email) -> Result<&User, UserStoreError>;
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError>;

    async fn check(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}