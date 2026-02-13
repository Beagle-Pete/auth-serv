use auth_service::{
    Application, 
    services::hashmap_user_store::HashmapUserStore,
    services::hashset_banned_token_store::HashsetBannedTokenStore,
    services::hahsmap_two_fa_code_store::HashMapTwoFACodeStore,
    app_state::TwoFACodeStoreType,
    app_state::AppState,
    utils::constants::test,
};

use axum::routing::head;

use std::{
    sync::Arc,
    collections::HashMap
};
use tokio::sync::RwLock;
use uuid::Uuid;
use reqwest::cookie::Jar;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token_store: Arc<RwLock<HashsetBannedTokenStore>>,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let two_fa_code_store = Arc::new(RwLock::new(HashMapTwoFACodeStore::default()));

        let app_state = AppState::new(user_store, banned_token_store.clone(), two_fa_code_store.clone());

        let app = Application::build(app_state,test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            http_client,
            banned_token_store,
            two_fa_code_store,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<T: serde::Serialize>(&self, body: &T) -> reqwest::Response {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<T: serde::Serialize>(&self, body: &T) -> reqwest::Response {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<T: serde::Serialize>(&self, body: &T) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

pub fn get_random_password() -> String {
    format!("{}", Uuid::new_v4())
}

pub fn parse_cookie_values(header_value: &str) -> HashMap<&str, &str>{
    // Parse through cookies from reqwest
    let parts: Vec<&str> = header_value.split(";").collect();

    let mut map = HashMap::new();
    for part in parts {
        let (name, value) = part.split_once("=").unwrap();
        map.insert(name.trim(), value.trim());
    }

    map
}

pub fn get_all_cookies(response: &reqwest::Response) -> HashMap<String, String> {
    let cookies: HashMap<String, String> = response
        .cookies()
        .map(|cookie| {
            let name = cookie.name().to_owned();
            let value = cookie.value().to_owned();
            (name, value)
        })
        .collect();

    cookies
}