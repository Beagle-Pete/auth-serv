use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::{AuthAPIError, User};
use crate::services::hashmap_user_store::UserStoreError;

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;
    let requires_2fa = request.requires_2fa;

    if !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store.add_user(user)
        .map_err(|err| 
            {
                match err {
                    UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
                    _ => AuthAPIError::UnexpectedError,
                }
            }
        )?;

    let response = Json(SignupRespose {
        message: "User created successfully!".to_owned(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupRespose {
    pub message: String,
}