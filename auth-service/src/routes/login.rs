use axum::{Json, response::IntoResponse, http::status::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::domain::{AuthAPIError};

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
