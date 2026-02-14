use axum::response::IntoResponse;
use axum::http::status::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::domain::AuthAPIError;

pub async fn verify_2fa(Json(request): Json<VerifyTwoFARequest>) -> Result<impl IntoResponse, AuthAPIError> {

    // Create JSON response body with 2FA
    // let response_json = Json(VerifyTwoFAResponse {
    //     email: "2FA required".to_owned(),
    //     login_attempt_id: "".to_owned(),
    //     two_fa_code: "".to_owned(),
    // });

    Ok(StatusCode::OK)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTwoFARequest {
    pub email: String,
    #[serde(rename = "loginAttempId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
