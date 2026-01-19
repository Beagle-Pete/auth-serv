use axum::response::IntoResponse;
use axum::http::status::StatusCode;

pub async fn verify_token() -> impl IntoResponse {
    StatusCode::OK.into_response()
}