use axum::response::IntoResponse;
use axum::http::status::StatusCode;
use axum::Json;
use axum::extract::State;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};
use crate::AppState;
use crate::utils::auth;

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<VerifyTwoFARequest>
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {

    // Verify request by parsing. If not valid return HTTP code 400
    let email = Email::parse(request.email)?;
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)?;
    let two_fa_code = TwoFACode::parse(request.two_fa_code)?;

    // Verify login attempt ID and 2FA code Are correct. If not valid return HTTP code 401
    let (login_attempt_id_true, two_fa_code_true) = {
        let two_fa_store = state.two_fa_code_store.read().await;
        let (login_attempt_id_true, two_fa_code_true) = two_fa_store.get_code(&email)
            .await
            .map_err(|_| AuthAPIError::IncorrectCredentials)?;
        (login_attempt_id_true, two_fa_code_true)
    };

    if (login_attempt_id != login_attempt_id_true) || (two_fa_code != two_fa_code_true) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    // Create JWT token
    let auth_cookie = auth::generate_auth_cookie(&email)
        .map_err(|_| {AuthAPIError::UnexpectedError})?;
    let updated_jar = jar.add(auth_cookie);

    // Remove 2FA code from store
    {
        let mut two_fa_store = state.two_fa_code_store.write().await;
        two_fa_store.remove_code(&email)
            .await
            .map_err(|_| AuthAPIError::UnexpectedError)?;
    }

    Ok((updated_jar, StatusCode::OK))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTwoFARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
