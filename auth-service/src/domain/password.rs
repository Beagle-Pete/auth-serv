use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use super::AuthAPIError;
use color_eyre::eyre::{Context, Result};
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct HashedPassword(SecretString);

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        self.0.expose_secret()
    }
}

impl PartialEq for HashedPassword {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl HashedPassword {
    pub async fn parse(password: SecretString) -> Result<Self, AuthAPIError> {

        if password.expose_secret().len() < 8 {
            return Err(AuthAPIError::InvalidCredentials);
        }

        let password_hash = compute_password_hash(&password)
            .await
            .map_err(AuthAPIError::UnexpectedError)?;

        Ok(Self(password_hash))
    }

    pub fn parse_password_hash(hash: SecretString) -> Result<Self, AuthAPIError>{
        match PasswordHash::new(hash.expose_secret()) {
            Ok(_) => Ok(Self(hash)),
            Err(e) => Err(AuthAPIError::UnexpectedError(e.into())),
        }
    }
    
    #[tracing::instrument(name = "Verify raw password", skip_all)]
    pub async fn verify_raw_password(
        &self,
        password_candidate: &SecretString,
    ) -> Result<()> {
        // Retrieve the current span from the tracing context
        // Span represents the execution context to verify the password
        let current_span = tracing::Span::current();

        let password_hash = self.as_ref().to_owned();
        let password_candidate = password_candidate.to_owned();
        
        tokio::task::spawn_blocking(move || {
            current_span.in_scope(|| {
                let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&password_hash)?;

                Argon2::default().verify_password(password_candidate.expose_secret().as_bytes(), &expected_password_hash)
                    .wrap_err("failed to verify password hash")
            })
        })
        .await?
    }
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: &SecretString) -> Result<SecretString> {
    // Retrieve the current span from the tracing context
    // Span represents the execution context for the compute_password_hash function
    let current_span = tracing::Span::current();

    let password = password.to_owned();

    tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut OsRng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.expose_secret().as_bytes(), &salt)?
            .to_string();
        
            Ok(SecretString::new(password_hash.into_boxed_str()))
        }) 
    })
    .await?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_password() {
        let raw_password = SecretString::new( "RustOrBust456!".to_owned().into_boxed_str());
        let raw_password_wrong = SecretString::new("RustOrBust4567!".to_owned().into_boxed_str());
        let password = HashedPassword::parse(raw_password.clone()).await.unwrap();

        assert!(password.verify_raw_password(&raw_password).await.is_ok());
        assert!(password.verify_raw_password(&raw_password_wrong).await.is_err());
    }

    #[tokio::test]
    async fn test_asref_impl() {
        let raw_password = SecretString::new( "RustOrBust456!".to_owned().into_boxed_str());
        let password1 = HashedPassword::parse(raw_password).await.unwrap();

        let raw_hased_password = SecretString::new( password1.as_ref().to_owned().into_boxed_str());
        let password2 = HashedPassword::parse_password_hash(raw_hased_password).unwrap();

        assert_eq!(password1, password2);
    }

    #[tokio::test]
    async fn test_invalid_password() {

        let test_cases = [
            HashedPassword::parse(SecretString::new( "1234567".to_owned().into_boxed_str())).await,
            HashedPassword::parse(SecretString::new( "badpass".to_owned().into_boxed_str())).await,
        ];

        for test_case in test_cases {
            assert_eq!(test_case, Err(AuthAPIError::InvalidCredentials));
        }
        
    }

    #[test]
    fn can_parse_valid_argon2_hash() {
        // Arrange - Create a valid Argon2 hash
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Act
        let hash_string_secret = SecretString::new(hash_string.clone().into_boxed_str());
        let hash_password = HashedPassword::parse_password_hash
            (hash_string_secret)
            .unwrap();

        // Assert
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
    }
}