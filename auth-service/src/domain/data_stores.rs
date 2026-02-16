use super::{User, Email, Password, LoginAttemptId, TwoFACode};

// User Store
#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;

    async fn get_user(&self, email: &Email) -> Result<&User, UserStoreError>;
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

// Banned Token Store
#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    InvalidToken,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError>;

    async fn check_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

// 2FA Store
#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttempIdNotFound,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}