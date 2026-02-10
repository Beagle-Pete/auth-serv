use axum::{Json, extract::State, http::status::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::AppState;
use crate::domain::AuthAPIError;
use crate::utils::auth;

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;

    let banned_token_store = state.banned_token_store;

    let _claims = auth::validate_token(&token, banned_token_store)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
