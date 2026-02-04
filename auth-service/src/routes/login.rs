use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domain::{AuthAPIError, UserStoreError, Email, Password};

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)?;
    let password = Password::parse(request.password)?;

    let user_store = state.user_store.write().await;
    
    if let Err(err) = user_store.validate_user(&email, &password).await {
        match err {
            UserStoreError::UserNotFound => return Err(AuthAPIError::IncorrectCredentials),
            UserStoreError::InvalidCredentials => return Err(AuthAPIError::IncorrectCredentials),
            _ => return Err(AuthAPIError::UnexpectedError)
        }
    }
    
    let user = user_store.get_user(&email).await
        .map_err(|err| {
            match err {
                UserStoreError::UserNotFound => AuthAPIError::InvalidCredentials,
                _ => AuthAPIError::UnexpectedError
            }
        })?;

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}