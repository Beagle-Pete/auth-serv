use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;
use color_eyre::eyre::{Context, Result};
use secrecy::{ExposeSecret, SecretString};

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    // TODO: Does this need to be in side an Arc<RwLock<...>> ? May be redundant since the banned token store
    // is already inside a Arc<RwLock<>>. Try implementing this without it. May need to change `check_token`
    // method to take in &mut self.
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    #[tracing::instrument(name = "Add_Token", skip_all)]
    async fn add_token(&mut self, token: &SecretString) -> Result<(), BannedTokenStoreError> {
        let key = get_key(token);

        let mut conn_lock = self.conn.write().await;

        conn_lock
            .set_ex(key, token.expose_secret(), TOKEN_TTL_SECONDS as u64)
            .wrap_err("failed to set banned token in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)
    }

    #[tracing::instrument(name = "Check_Token", skip_all)]
    async fn check_token(&self, token: &SecretString) -> Result<bool, BannedTokenStoreError> {
        let key = get_key(token);

        let mut conn_lock = self.conn.write().await;
        conn_lock.exists(key)
            .wrap_err("failed to check if token exists in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &SecretString) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token.expose_secret())
}