mod routes;
pub mod domain;
pub mod services;
pub mod app_state;
pub mod utils;

use routes as api_routes;
use app_state::AppState;
use utils::constants::{DROPLET_IP, JWT_SECRET};

use std::error::Error;

use axum::{Router, routing::post, serve::Serve, http::Method};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, cors::CorsLayer};

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

        let allowed_origins = [
            "http://localhost:8001".parse()?,
            format!("http://{}:8001", DROPLET_IP.as_str()).parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .fallback_service(assets_dir)
            .route("/signup", post(api_routes::signup))
            .route("/login", post(api_routes::login))
            .route("/logout", post(api_routes::logout))
            .route("/verify-2fa", post(api_routes::verify_2fa))
            .route("/verify-token", post(api_routes::verify_token))
            .with_state(app_state)
            .layer(cors);

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


