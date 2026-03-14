use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;
use secrecy::SecretString;

lazy_static! {
    pub static ref JWT_SECRET: SecretString = SecretString::new(set_token(env::JWT_SECRET_ENV_VAR).into_boxed_str());
    pub static ref DROPLET_IP: String = set_token(env::DROPLET_IP_ENV_VAR);
    pub static ref DATABASE_URL: SecretString = SecretString::new(set_token(env::DATABASE_URL_ENV_VAR).into_boxed_str());
    pub static ref REDIS_HOST_NAME: String = set_token_with_default(env::REDIS_HOST_NAME_ENV_VAR, DEFAULT_REDIS_HOSTNAME);
    pub static ref POSTMARK_AUTH_TOKEN: SecretString = SecretString::new(set_token(env::POSTMARK_AUTH_TOKEN_ENV_VAR).into_boxed_str());
}

// TODO: Modify to return a SecretString
fn set_token(var_name: &str) -> String {
    dotenv().ok();
    let secret = std_env::var(var_name).unwrap_or_else(|_| panic!("{} must be set.", var_name));
    if secret.is_empty() {
        panic!("{var_name} must not be empty.");
    }
    secret
}

fn set_token_with_default(var_name: &str, default: &str) -> String {
    dotenv().ok();
    let secret = std_env::var(var_name).unwrap_or(default.to_owned());
    if secret.is_empty() {
        panic!("{var_name} must not be empty.");
    }
    secret
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DROPLET_IP_ENV_VAR: &str = "DROPLET_IP";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
    pub const POSTMARK_AUTH_TOKEN_ENV_VAR: &str = "POSTMARK_AUTH_TOKEN";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";

    pub mod email_client {
        use std::time::Duration;

        pub const BASE_URL: &str = "https://api.postmarkapp.com/email";
        pub const SENDER: &str = "ehamil@puppymailbox.com";
        pub const TIMEOUT: Duration = Duration::from_secs(10);
    }
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";

    pub mod email_client {
        use std::time::Duration;

        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = Duration::from_millis(200);
    }
}

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";