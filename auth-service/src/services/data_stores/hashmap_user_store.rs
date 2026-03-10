use crate::domain::{data_stores::UserStore, data_stores::UserStoreError, User, Email};
use color_eyre::eyre::Result;
use secrecy::SecretString;

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

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;
        Ok(user.clone())
    }
    
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        let password_secret = SecretString::new(raw_password.to_owned().into_boxed_str());
        user.password
            .verify_raw_password(&password_secret)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use super::*;
    use crate::domain::HashedPassword;

    #[tokio::test]
    async fn test_add_user() {
        let mut users = HashmapUserStore::default();

        let email_secret = SecretString::new("text@example.com".to_owned().into_boxed_str());
        let email = Email::parse(email_secret).unwrap();

        let password_secret = SecretString::new("1234ABCD".to_owned().into_boxed_str());
        let password = HashedPassword::parse(password_secret).await.unwrap();

        let new_user = User::new(email, password, false);

        let add_new_user1 = users.add_user(new_user.clone()).await;
        let add_new_user2 = users.add_user(new_user.clone()).await;

        assert_eq!(add_new_user1, Ok(()));
        assert_eq!(add_new_user2, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut users = HashmapUserStore::default();

        let email_secret = SecretString::new("text@example.com".to_owned().into_boxed_str());
        let email = Email::parse(email_secret).unwrap();

        let password_secret = SecretString::new("1234ABCD".to_owned().into_boxed_str());
        let password = HashedPassword::parse(password_secret).await.unwrap();
        let new_user = User::new(email, password, false);

        let _ = users.add_user(new_user.clone()).await;

        let get_user1 = users.get_user(&new_user.email).await;

        let email2_secret = SecretString::new("non-existent-user@example.com".to_owned().into_boxed_str());
        let email2 = Email::parse(email2_secret).unwrap();
        let get_user2 = users.get_user(&email2).await;

        assert_eq!(get_user1, Ok(new_user));
        assert_eq!(get_user2, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut users = HashmapUserStore::default();

        let email_secret = SecretString::new("text@example.com".to_owned().into_boxed_str());
        let email = Email::parse(email_secret).unwrap();

        let password_secret = SecretString::new("1234ABCD".to_owned().into_boxed_str());
        let password = HashedPassword::parse(password_secret.clone()).await.unwrap();
        let new_user = User::new(email, password, false);

        let _ = users.add_user(new_user.clone()).await;

        let validate_user1 = users.validate_user(&new_user.email, &password_secret.expose_secret().as_ref()).await;

        let raw_password2 = "wrong_password".to_owned();
        let validate_user2 = users.validate_user(&new_user.email, &raw_password2).await;

        let email3_secret = SecretString::new("non-existent-user@example.com".to_owned().into_boxed_str());
        let email3 = Email::parse(email3_secret).unwrap();
        let validate_user3 = users.validate_user(&email3, &password_secret.expose_secret().as_ref()).await;

        assert_eq!(validate_user1, Ok(()));
        assert_eq!(validate_user2, Err(UserStoreError::InvalidCredentials));
        assert_eq!(validate_user3, Err(UserStoreError::UserNotFound));
    }
}