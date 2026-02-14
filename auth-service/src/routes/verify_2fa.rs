use axum::response::IntoResponse;
use axum::http::status::StatusCode;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};
use crate::AppState;

pub async fn verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<VerifyTwoFARequest>
) -> Result<impl IntoResponse, AuthAPIError> {

    // Verify request by parsing. If not valid return HTTP code 400
    let email = Email::parse(request.email)?;
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)?;
    let two_fa_code = TwoFACode::parse(request.two_fa_code)?;

    // Verify login attempt ID and 2FA code Are correct. If not valid return HTTP code 401
    let two_fa_store = state.two_fa_code_store.read().await;
    let (login_attempt_id_true, two_fa_code_true) = two_fa_store.get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    if (login_attempt_id != login_attempt_id_true) || (two_fa_code != two_fa_code_true) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(StatusCode::OK)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTwoFARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
