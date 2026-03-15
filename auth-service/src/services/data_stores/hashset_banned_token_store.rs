use crate::domain::{data_stores::BannedTokenStore, data_stores::BannedTokenStoreError};
use secrecy::{ExposeSecret, SecretString};

use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: &SecretString) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.expose_secret().into());

        Ok(())
    }

    async fn check_token(&self, token: &SecretString) -> Result<bool, BannedTokenStoreError> {        
        Ok(self.tokens.contains(token.expose_secret()))
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();

        let token1 = SecretString::new("token1".to_owned().into_boxed_str());
        let token2 = SecretString::new("token2".to_owned().into_boxed_str());

        banned_tokens.add_token(&token1).await.unwrap();
        banned_tokens.add_token(&token2).await.unwrap();
        banned_tokens.add_token(&token2).await.unwrap();

        assert!(banned_tokens.tokens.contains(token1.expose_secret()));
        assert!(banned_tokens.tokens.contains(token2.expose_secret()));
    }

    #[tokio::test]
    async fn test_get_existing_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();

        let token1 = SecretString::new("token1".to_owned().into_boxed_str());
        let token2 = SecretString::new("".to_owned().into_boxed_str());

        banned_tokens.add_token(&token1).await.unwrap();
        banned_tokens.add_token(&token2).await.unwrap();

        assert_eq!(banned_tokens.check_token(&token1).await, Ok(true));
        assert_eq!(banned_tokens.check_token(&token2).await, Ok(true));
    }

    #[tokio::test]
    async fn test_get_nonexisting_token() {
        let banned_tokens = HashsetBannedTokenStore::default();

        let token1 = SecretString::new("token1".to_owned().into_boxed_str());
        let token2 = SecretString::new("".to_owned().into_boxed_str());

        assert_eq!(banned_tokens.check_token(&token1).await, Ok(false));
        assert_eq!(banned_tokens.check_token(&token2).await, Ok(false));
    }
}