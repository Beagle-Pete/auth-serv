use crate::domain::{BannedTokenStore, BannedTokenStoreError};

use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.into());

        Ok(())
    }

    async fn check_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {        
        Ok(self.tokens.contains(token))
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();

        let token1 = "token1".to_owned();
        let token2 = "token2".to_owned();

        banned_tokens.add_token(&token1).await.unwrap();
        banned_tokens.add_token(&token2).await.unwrap();
        banned_tokens.add_token(&token2).await.unwrap();

        assert!(banned_tokens.tokens.contains(&token1));
        assert!(banned_tokens.tokens.contains(&token2));
    }

    #[tokio::test]
    async fn test_get_existing_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();

        let token1 = "token1".to_owned();
        let token2 = "".to_owned();

        banned_tokens.add_token(&token1).await.unwrap();
        banned_tokens.add_token(&token2).await.unwrap();

        assert_eq!(banned_tokens.check_token(&token1).await, Ok(true));
        assert_eq!(banned_tokens.check_token(&token2).await, Ok(true));
    }

    #[tokio::test]
    async fn test_get_nonexisting_token() {
        let banned_tokens = HashsetBannedTokenStore::default();

        let token1 = "token1".to_owned();
        let token2 = "".to_owned();

        assert_eq!(banned_tokens.check_token(&token1).await, Ok(false));
        assert_eq!(banned_tokens.check_token(&token2).await, Ok(false));
    }
}