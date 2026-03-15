use sqlx::PgPool;
use color_eyre::eyre::Result;
use secrecy::SecretString;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, HashedPassword, User,
};

#[derive(Debug)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Add user to database. 
        // If user already exists then do nothing and return NONE
        let result = sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            ON CONFLICT (email) DO NOTHING
            RETURNING email
            "#,
            &user.email.as_ref(), 
            &user.password.as_ref(), 
            &user.requires_2fa,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        // TODO: Should logic to check if user exist live in the signup route?
        match result {
            Some(_) => Ok(()),
            None => Err(UserStoreError::UserAlreadyExists),
        }
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let result = sqlx::query!(
            r#"
            SELECT email, password_hash, requires_2fa
            FROM users
            WHERE email = $1
            "#,
            email.as_ref(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        match result {
            Some(row) => {
                let email_secret = SecretString::new(row.email.into_boxed_str());
                let email = Email::parse(email_secret)
                    .map_err(UserStoreError::UnexpectedError)?;

                let password_secret = SecretString::new(row.password_hash.into_boxed_str());
                let password = HashedPassword::parse_password_hash(password_secret)
                    .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;
                let user = User::new(email, password, row.requires_2fa);
                Ok(user)
            },
            None => Err(UserStoreError::UserNotFound),
        }
    }
    

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        let password_secret = SecretString::new(raw_password.to_owned().into_boxed_str());
        user.password
            .verify_raw_password(&password_secret)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}