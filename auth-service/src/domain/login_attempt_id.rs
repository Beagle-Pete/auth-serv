use color_eyre::eyre::{Result, eyre};
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct LoginAttemptId(SecretString);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self> {
        if uuid::Uuid::try_parse(&id).is_err() {
            return Err(eyre!("Invalid login attempt ID"));
        }

        Ok(LoginAttemptId(SecretString::new(id.into_boxed_str())))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let new_uuid = uuid::Uuid::new_v4().to_string();
        LoginAttemptId(SecretString::new(new_uuid.into_boxed_str()))
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

impl PartialEq for LoginAttemptId {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}