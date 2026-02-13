use super::{User, Email, Password};

use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    InvalidToken,
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

    async fn check_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

// 2FA Store
#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttempIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        if uuid::Uuid::try_parse(&id).is_err() {
            return Err("Login Attempt ID not a valid".to_owned());
        }

        Ok(LoginAttemptId(id))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        LoginAttemptId(uuid::Uuid::new_v4().into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        match code.parse::<usize>() {
            Ok(_) => {
                if code.len() != 6 {
                    return Err("2FA code must be 6 digits".to_owned());
                }

                Ok(TwoFACode(code))
            },
            Err(_) => Err("Code ".to_owned()),
        }        
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut code = rand::rng().random_range(0..=999_999).to_string();
        
        if code.len() < 6 {
            let leading_zeros = "0".repeat(6-code.len());
            code = format!("{}{}", leading_zeros, code);
        } 

        TwoFACode(code)
    }
}