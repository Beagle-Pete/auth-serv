use super::AuthAPIError;

#[derive(Debug, Clone, PartialEq)]
struct Email (String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn parse(email: String) -> Result<Self, AuthAPIError> {

        // email must have @
        if !email.contains("@") {
            return Err(AuthAPIError::InvalidCredentials);
        }

        Ok(Self(email))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_input() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();

        assert_eq!(email.0, "test@example.com".to_owned());
    }

    #[tokio::test]
    async fn test_asref_impl() {
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let email = email.as_ref();

        assert_eq!(email, "test@example.com");
    }

    #[tokio::test]
    async fn test_invalid_input() {

        let test_cases = [
            Email::parse("testexample.com".to_owned()),
        ];

        for test_case in test_cases {
            assert_eq!(test_case, Err(AuthAPIError::InvalidCredentials));
        }
    }
}