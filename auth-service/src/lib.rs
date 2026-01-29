mod routes;
mod domain;
pub mod services;
pub mod app_state;

use routes as api_routes;
use app_state::AppState;
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};

use std::error::Error;

use axum::{Json, Router, http::status::StatusCode, response::{IntoResponse, Response}, routing::post, serve::Serve};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let assets_dir = ServeDir::new("assets");
        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(api_routes::signup))
            .route("/login", post(api_routes::login))
            .route("/logout", post(api_routes::logout))
            .route("/verify-2fa", post(api_routes::verify_2fa))
            .route("/verify-token", post(api_routes::verify_token))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);
        
        Ok(
            Self {
                server,
                address,
            }
        )
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErroMessage {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpetected error"),
        };

        let body = Json(ErroMessage {
            error: message.to_string(),
        });

        (status, body).into_response()
    }
}