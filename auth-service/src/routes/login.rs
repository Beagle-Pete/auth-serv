use axum::response::IntoResponse;
use axum::http::status::StatusCode;

pub async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}