use axum::response::IntoResponse;
use axum::http::status::StatusCode;

pub async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}