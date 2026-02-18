use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::{AuthAPIError, data_stores::UserStoreError, Email, HashedPassword, LoginAttemptId, TwoFACode};
use crate::utils::auth;

pub async fn login(
    State(state): State<AppState>, 
    jar: CookieJar,
    Json(request): Json<LoginRequest>
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    
    let email = Email::parse(request.email)?;
    HashedPassword::parse(request.password.clone()).await?;

    let user_store = state.user_store.write().await;
    
    if let Err(err) = user_store.validate_user(&email, &request.password).await {
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
        true => handle_2fa(&email, &state, jar.clone()).await,
        false => handle_no_2fa(&email, jar.clone()).await,
    }?;

    Ok((res1, (res2, res3.into_response())))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

async fn handle_2fa(email: &Email, state: &AppState, jar: CookieJar) -> Result<(CookieJar, StatusCode, Json<LoginResponse>), AuthAPIError> {

    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    // Send 2FA email
    let email_client = state.email_client.write().await;
    let subject= "Code";
    let content = two_fa_code.as_ref();
    email_client.send_email(email.clone(), subject, content)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    // Add 2FA code to store
    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    two_fa_code_store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;
    
    // Create JSON response body with 2FA
    let response_json = TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    };
    let response = LoginResponse::TwoFactorAuth(response_json);

    Ok((jar, StatusCode::PARTIAL_CONTENT, response.into()))
}

async fn handle_no_2fa(email: &Email, jar: CookieJar) -> Result<(CookieJar, StatusCode, Json<LoginResponse>), AuthAPIError> {
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