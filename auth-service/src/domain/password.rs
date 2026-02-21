use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use std::error::Error;
use super::AuthAPIError;

#[derive(Debug, Clone, PartialEq)]
pub struct HashedPassword(String);

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl HashedPassword {
    pub async fn parse(password: String) -> Result<Self, AuthAPIError> {

        if password.len() < 8 {
            return Err(AuthAPIError::InvalidCredentials);
        }

        let password_hash = compute_password_hash(&password)
            .await
            .map_err(|_| AuthAPIError::UnexpectedError)?;

        Ok(Self(password_hash))
    }

    pub fn parse_password_hash(hash: String) -> Result<Self, String>{
        match PasswordHash::new(&hash) {
            Ok(_) => Ok(Self(hash)),
            Err(_) => Err("tt".to_owned()),
        }
    }
    
    pub async fn verify_raw_password(
        &self,
        password_candidate: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
    let password_hash = self.as_ref().to_owned();
    let password_candidate = password_candidate.to_owned();
    
    tokio::task::spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&password_hash)?;
        Argon2::default().verify_password( password_candidate.as_bytes(), &expected_password_hash)
              .map_err(|e| e.into())
    }).await?
    }
}

async fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let password = password.to_owned();

    let password_hash = tokio::task::spawn_blocking(move || -> Result<String, Box<dyn Error + Send + Sync>>  {
        let salt: SaltString = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    
        Ok(password_hash)
    }).await??;

    Ok(password_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_password() {
        let raw_password = "RustOrBust456!".to_owned();
        let raw_password_wrong = "RustOrBust4567!".to_owned();
        let password = HashedPassword::parse(raw_password.clone()).await.unwrap();

        assert!(password.verify_raw_password(&raw_password).await.is_ok());
        assert!(password.verify_raw_password(&raw_password_wrong).await.is_err());
    }

    #[tokio::test]
    async fn test_asref_impl() {
        let password1 = HashedPassword::parse("RustOrBust456!".to_owned()).await.unwrap();
        let password2 = HashedPassword::parse_password_hash(password1.as_ref().to_owned()).unwrap();

        assert_eq!(password1, password2);
    }

    #[tokio::test]
    async fn test_invalid_password() {

        let test_cases = [
            HashedPassword::parse("1234567".to_owned()).await,
            HashedPassword::parse("badpass".to_owned()).await,
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
        let hash_password = HashedPassword::parse_password_hash
            (hash_string.clone())
            .unwrap();

        // Assert
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
    }
}