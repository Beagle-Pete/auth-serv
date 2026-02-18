use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::{AuthAPIError, User, UserStoreError, Email, HashedPassword};

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)?;
    let password = HashedPassword::parse(request.password).await?;
    let requires_2fa = request.requires_2fa;

    let user = User::new(email, password, requires_2fa);
    let mut user_store = state.user_store.write().await;

    user_store.add_user(user).await
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