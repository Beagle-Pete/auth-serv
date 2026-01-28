use core::panic::PanicMessage;
use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let user_exists = self.users.contains_key(&user.email);
        match user_exists {
            true => {
                Err(UserStoreError::UserAlreadyExists)
            },
            false => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            },
        }
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }
    
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;

        if user.password != password {
            return Err(UserStoreError::InvalidCredentials)
        }

        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut users = HashmapUserStore::default();

        let new_user = User{
            email: "text@example.com".to_owned(),
            password: "123ABC".to_owned(),
            requires_2fa: false,
        };

        let add_new_user1 = users.add_user(new_user.clone());
        let add_new_user2 = users.add_user(new_user.clone());

        assert_eq!(add_new_user1, Ok(()));
        assert_eq!(add_new_user2, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut users = HashmapUserStore::default();

        let new_user = User{
            email: "text@example.com".to_owned(),
            password: "123ABC".to_owned(),
            requires_2fa: false,
        };
        let _ = users.add_user(new_user.clone());

        let get_user1 = users.get_user(&new_user.email);
        let get_user2 = users.get_user("non-existent-user@example.com");

        assert_eq!(get_user1, Ok(&new_user));
        assert_eq!(get_user2, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut users = HashmapUserStore::default();

        let new_user = User{
            email: "text@example.com".to_owned(),
            password: "123ABC".to_owned(),
            requires_2fa: false,
        };
        let _ = users.add_user(new_user.clone());

        let validate_user1 = users.validate_user(&new_user.email, &new_user.password);
        let validate_user2 = users.validate_user(&new_user.email, "wrong_password");
        let validate_user3 = users.validate_user("non-existent-user@example.com", &new_user.password);

        assert_eq!(validate_user1, Ok(()));
        assert_eq!(validate_user2, Err(UserStoreError::InvalidCredentials));
        assert_eq!(validate_user3, Err(UserStoreError::UserNotFound));
    }
}