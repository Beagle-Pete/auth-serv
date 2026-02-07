use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domain::{AuthAPIError, UserStoreError, Email, Password};
use crate::utils::auth;

pub async fn login(
    State(state): State<AppState>, 
    jar: CookieJar,
    Json(request): Json<LoginRequest>
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    
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

    let auth_cookie = auth::generate_auth_cookie(&email)
        .map_err(|_| {AuthAPIError::UnexpectedError})?;

    let updated_jar = jar.add(auth_cookie);

    Ok((updated_jar, StatusCode::OK.into_response()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}