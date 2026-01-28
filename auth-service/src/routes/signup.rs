use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::User;

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> impl IntoResponse {
    let user = User::new(request.email, request.password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;
    user_store.add_user(user).unwrap();

    let response = Json(SignupRespose {
        message: "User created successfully!".to_owned(),
    });

    (StatusCode::CREATED, response)
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