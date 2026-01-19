use axum::response::IntoResponse;
use axum::http::status::StatusCode;

pub async fn signup() -> impl IntoResponse {
    StatusCode::OK.into_response()
}