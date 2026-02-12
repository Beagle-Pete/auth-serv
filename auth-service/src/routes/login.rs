use axum::http::response;
use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

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

    let (res1, res2, res3) = match user.requires_2fa {
        true => handle_2fa(jar.clone()).await,
        false => hadle_no_2fa(&email, jar.clone()).await,
    }?;

    Ok((res1, (res2, res3.into_response())))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

async fn handle_2fa(jar: CookieJar) -> Result<(CookieJar, StatusCode, Json<LoginResponse>), AuthAPIError> {
    
    let response_json = TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: "123456".to_owned(),
    };
    let response = LoginResponse::TwoFactorAuth(response_json);

    Ok((jar, StatusCode::PARTIAL_CONTENT, response.into()))
}

async fn hadle_no_2fa(email: &Email, jar: CookieJar) -> Result<(CookieJar, StatusCode, Json<LoginResponse>), AuthAPIError> {
    let auth_cookie = auth::generate_auth_cookie(email)
        .map_err(|_| {AuthAPIError::UnexpectedError})?;

    let updated_jar = jar.add(auth_cookie);

    let response = LoginResponse::RegularAuth;

    Ok((updated_jar, StatusCode::OK, response.into()))
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}