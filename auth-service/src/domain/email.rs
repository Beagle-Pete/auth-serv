use super::AuthAPIError;

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