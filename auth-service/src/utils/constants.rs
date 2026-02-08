use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref JWT_SECRET: String = set_token(env::JWT_SECRET_ENV_VAR);
    pub static ref DROPLET_IP: String = set_token(env::DROPLET_IP_ENV_VAR);
}

fn set_token(var_name: &str) -> String {
    dotenv().ok();
    let secret = std_env::var(var_name).unwrap_or_else(|_| panic!("{} must be set.", var_name));
    if secret.is_empty() {
        panic!("{var_name} must not be empty.");
    }
    println!("env var - name: {var_name}, value: {secret}");
    secret
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DROPLET_IP_ENV_VAR: &str = "DROPLET_IP";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

pub const JWT_COOKIE_NAME: &str = "jwt";