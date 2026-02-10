use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::Deserialize;

use crate::utils::auth;
use crate::domain::AuthAPIError;

pub async fn verify_token(Json(request): Json<VerifyTokenRequest>) -> Result<impl IntoResponse, AuthAPIError> {

    let token = request.token;

    auth::validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;


    Ok(StatusCode::OK.into_response())
}


#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}