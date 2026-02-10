use crate::domain::{BannedTokenStore, BannedTokenStoreError};

use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn store_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        self.tokens.insert(token.into());

        Ok(())
    }

    async fn check(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        
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

        banned_tokens.store_token(&token1).await.unwrap();
        banned_tokens.store_token(&token2).await.unwrap();
        banned_tokens.store_token(&token2).await.unwrap();

        assert!(banned_tokens.tokens.contains(&token1));
        assert!(banned_tokens.tokens.contains(&token2));
    }

    #[tokio::test]
    async fn test_get_existing_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();

        let token1 = "token1".to_owned();
        let token2 = "".to_owned();

        banned_tokens.store_token(&token1).await.unwrap();
        banned_tokens.store_token(&token2).await.unwrap();

        assert!(banned_tokens.check(&token1).await.unwrap());
        assert!(banned_tokens.check(&token2).await.unwrap());
    }

    #[tokio::test]
    async fn test_get_nonexisting_token() {
        let mut banned_tokens = HashsetBannedTokenStore::default();

        let token1 = "token1".to_owned();
        let token2 = "".to_owned();

        assert!(!banned_tokens.check(&token1).await.unwrap());
        assert!(!banned_tokens.check(&token2).await.unwrap());
    }
}