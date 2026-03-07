use axum::{Json, http::status::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use color_eyre::eyre::Report;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("Use already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Invalid Login Attempt ID")]
    InvalidLoginAttempId,
    #[error("Invalid 2FA Code")]
    InvalidTwoFACode,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);

        let (status, message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthAPIError::InvalidLoginAttempId => (StatusCode::BAD_REQUEST, "Invalid login attempt id"),
            AuthAPIError::InvalidTwoFACode => (StatusCode::BAD_REQUEST, "Invalid 2FA code"),
            AuthAPIError::UnexpectedError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpetected error"),
        };

        let body = Json(ErrorResponse {
            error: message.to_string(),
        });

        (status, body).into_response()
    }
}

impl PartialEq for AuthAPIError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::IncorrectCredentials, Self::IncorrectCredentials)
                | (Self::MissingToken, Self::MissingToken)
                | (Self::InvalidToken, Self::InvalidToken)
                | (Self::InvalidLoginAttempId, Self::InvalidLoginAttempId)
                | (Self::InvalidTwoFACode, Self::InvalidTwoFACode)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn log_error_chain(e: &(dyn std::error::Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}