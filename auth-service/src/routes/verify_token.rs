use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::Deserialize;

use crate::utils::auth;
use crate::domain::AuthAPIError;
use crate::AppState;

pub async fn verify_token(
    State(state): State<AppState>, 
    Json(request): Json<VerifyTokenRequest>
) -> Result<impl IntoResponse, AuthAPIError> {

    let token = request.token;

    auth::validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    // Check if token is in banned token store
    let banned_token_store = state.banned_token_store.write().await;
    let is_token_banned = banned_token_store.check(&token)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    if is_token_banned {
        return Err(AuthAPIError::InvalidToken);
    }

    Ok(StatusCode::OK.into_response())
}


#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}