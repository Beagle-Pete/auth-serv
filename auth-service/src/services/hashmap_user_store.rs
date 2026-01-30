use crate::domain::{UserStore, UserStoreError, User, Email, Password};

use core::panic::PanicMessage;
use std::collections::HashMap;


#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
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

    async fn get_user(&self, email: &Email) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }
    
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        if &user.password != password {
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

        let email = Email::parse("text@example.com".to_owned()).unwrap();
        let password = Password::parse("1234ABCD".to_owned()).unwrap();

        let new_user = User::new(email, password, false);

        let add_new_user1 = users.add_user(new_user.clone()).await;
        let add_new_user2 = users.add_user(new_user.clone()).await;

        assert_eq!(add_new_user1, Ok(()));
        assert_eq!(add_new_user2, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut users = HashmapUserStore::default();

        let email = Email::parse("text@example.com".to_owned()).unwrap();
        let password = Password::parse("1234ABCD".to_owned()).unwrap();
        let new_user = User::new(email, password, false);

        let _ = users.add_user(new_user.clone()).await;

        let get_user1 = users.get_user(&new_user.email).await;

        let email2 = Email::parse("non-existent-user@example.com".to_owned()).unwrap();
        let get_user2 = users.get_user(&email2).await;

        assert_eq!(get_user1, Ok(&new_user));
        assert_eq!(get_user2, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut users = HashmapUserStore::default();

        let email = Email::parse("text@example.com".to_owned()).unwrap();
        let password = Password::parse("1234ABCD".to_owned()).unwrap();
        let new_user = User::new(email, password, false);

        let _ = users.add_user(new_user.clone()).await;

        let validate_user1 = users.validate_user(&new_user.email, &new_user.password).await;

        let password2 = Password::parse("wrong_password".to_owned()).unwrap();
        let validate_user2 = users.validate_user(&new_user.email, &password2).await;

        let email3 = Email::parse("non-existent-user@example.com".to_owned()).unwrap();
        let validate_user3 = users.validate_user(&email3, &new_user.password).await;

        assert_eq!(validate_user1, Ok(()));
        assert_eq!(validate_user2, Err(UserStoreError::InvalidCredentials));
        assert_eq!(validate_user3, Err(UserStoreError::UserNotFound));
    }
}