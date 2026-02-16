use super::AuthAPIError;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, AuthAPIError> {
        if uuid::Uuid::try_parse(&id).is_err() {
            return Err(AuthAPIError::InvalidLoginAttempId);
        }

        Ok(LoginAttemptId(id))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        LoginAttemptId(uuid::Uuid::new_v4().into())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}