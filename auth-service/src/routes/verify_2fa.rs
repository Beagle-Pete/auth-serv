use axum::response::IntoResponse;
use axum::http::status::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode};

pub async fn verify_2fa(Json(request): Json<VerifyTwoFARequest>) -> Result<impl IntoResponse, AuthAPIError> {

    let email = Email::parse(request.email)?;
    let login_attemp_id = LoginAttemptId::parse(request.login_attempt_id)?;
    let two_fa_code = TwoFACode::parse(request.two_fa_code)?;

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
