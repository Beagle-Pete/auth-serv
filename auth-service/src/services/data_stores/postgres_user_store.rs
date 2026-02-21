use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, HashedPassword, User,
};

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
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Add user to database. 
        // If user already exists then do nothing adn return NONE
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
        .map_err(|_| UserStoreError::UnexpectedError)?;

        // TODO: Should logic to check if user exist live in the signup route?
        match result {
            Some(_) => Ok(()),
            None => Err(UserStoreError::UserAlreadyExists),
        }
    }

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
        .map_err(|_| UserStoreError::UnexpectedError)?;

        match result {
            Some(row) => {
                let email = Email::parse(row.email)
                    .map_err(|_| UserStoreError::UnexpectedError)?;
                let password = HashedPassword::parse_password_hash(row.password_hash)
                    .map_err(|_| UserStoreError::UnexpectedError)?;
                let user = User::new(email, password, row.requires_2fa);
                Ok(user)
            },
            None => Err(UserStoreError::UserNotFound),
        }
    }
    
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        user.password
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
}