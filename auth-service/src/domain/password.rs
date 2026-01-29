use super::AuthAPIError;

#[derive(Debug, Clone, PartialEq)]
struct Password(String);

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Password {
    pub fn parse(password: String) -> Result<Self, AuthAPIError> {

        if password.len() < 8 {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(Self(password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_password() {
        let password = Password::parse("RustOrBust456!".to_owned()).unwrap();

        assert_eq!(password.0, "RustOrBust456!".to_owned());
    }

    #[tokio::test]
    async fn test_asref_impl() {
        let password = Password::parse("RustOrBust456!".to_owned()).unwrap();

        assert_eq!(password.as_ref(), "RustOrBust456!");
    }

    #[tokio::test]
    async fn test_invalid_password() {

        let test_cases = [
            Password::parse("1234567".to_owned()),
            Password::parse("badpass".to_owned()),
        ];

        for test_case in test_cases {
            assert_eq!(test_case, Err(AuthAPIError::InvalidCredentials));
        }
        
    }
}