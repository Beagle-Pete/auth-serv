use crate::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
};

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HashMapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {

        self.codes.insert(email, (login_attempt_id, code));

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email)
            .ok_or(TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((login_attempt_id, two_fa_code)) => Ok((login_attempt_id.clone(), two_fa_code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttempIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut two_fa_codes = HashMapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();


        let result = two_fa_codes.add_code(email.clone(), login_attempt_id, code).await;

        assert!(result.is_ok());
        assert_eq!(two_fa_codes.codes.len(), 1);
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut two_fa_codes = HashMapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = two_fa_codes.add_code(email.clone(), login_attempt_id, code).await;

        assert!(result.is_ok());
        assert_eq!(two_fa_codes.codes.len(), 1);

        let result = two_fa_codes.remove_code(&email).await;
        assert!(result.is_ok());
        assert_eq!(two_fa_codes.codes.len(), 0);
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut two_fa_codes = HashMapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = two_fa_codes.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await;

        assert!(result.is_ok());
        assert_eq!(two_fa_codes.codes.len(), 1);

        let (login_attempt_id2, code2) = two_fa_codes.get_code(&email).await.unwrap();
        assert_eq!(login_attempt_id, login_attempt_id2);
        assert_eq!(code, code2);
    }
}