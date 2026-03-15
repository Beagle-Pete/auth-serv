use color_eyre::eyre::{Result, eyre};
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct Email(SecretString);

// TODO: Does exposing the secret here defeat the purpose of using SecretString?
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.expose_secret()
    }
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for Email {}

impl std::hash::Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl Email {
    pub fn parse(email: SecretString) -> Result<Self> {

        // email must have @
        if !email.expose_secret().contains("@") {
            return Err(eyre!("failed to parse email"));
        }

        Ok(Self(email))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_input() {
        let raw_email = SecretString::new("test@example.com".to_owned().into_boxed_str());
        let email = Email::parse(raw_email).unwrap();

        assert_eq!(email.0.expose_secret(), "test@example.com".to_owned());
    }

    #[tokio::test]
    async fn test_asref_impl() {
        let raw_email = SecretString::new("test@example.com".to_owned().into_boxed_str());
        let email = Email::parse(raw_email).unwrap();
        let email = email.as_ref();

        assert_eq!(email, "test@example.com");
    }

    #[tokio::test]
    async fn test_invalid_input() {

        let test_cases = [
            Email::parse(SecretString::new("testexample.com".to_owned().into_boxed_str())),
        ];

        for test_case in test_cases {
            assert!(test_case.is_err());
            // assert_eq!(test_case, Err(AuthAPIError::InvalidCredentials));
        }
    }
}