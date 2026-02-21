use auth_service::{
    services::data_stores::hashset_banned_token_store::HashsetBannedTokenStore,
    utils::{auth, constants::JWT_COOKIE_NAME},
};

use reqwest::{Url, cookie::CookieStore};
use tokio::sync::RwLock;

use std::sync::Arc;

use crate::helpers::{TestApp, get_random_email, parse_cookie_values};

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    // Signup
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    //  Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie_login = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie_login.value().is_empty());

    // Logout
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie_logout = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    if let Some(cookie) = &auth_cookie_logout {
        assert_eq!(cookie.value(), "");
    }

    let banned_token_store = app.banned_token_store.write().await;
    let is_token_banned = banned_token_store
        .check_token(auth_cookie_login.value())
        .await
        .unwrap();
    drop(banned_token_store);

    assert!(is_token_banned);

    app.delete_database(&app.db_name.clone()).await;
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    // Signup
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    // Logout
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME);

    if let Some(cookie) = &auth_cookie {
        assert_eq!(cookie.value(), "");
    }

    // Logout again
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);

    app.delete_database(&app.db_name.clone()).await;
}

#[tokio::test]
async fn should_return_400_if_there_are_no_cookies() {
    let mut app = TestApp::new().await;

    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    let cookies = app.cookie_jar.cookies(url);

    assert!(cookies.is_none());

    app.delete_database(&app.db_name.clone()).await;
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let mut app = TestApp::new().await;

    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    // add a cookie
    app.cookie_jar.add_cookie_str(
        "random=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
        url,
    );

    let cookies = app.cookie_jar.cookies(url).unwrap();
    let cookies = parse_cookie_values(cookies.to_str().unwrap());

    let cookie_exists = cookies.contains_key(JWT_COOKIE_NAME);

    assert!(!cookie_exists);

    app.delete_database(&app.db_name.clone()).await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        url,
    );

    let cookies = app.cookie_jar.cookies(url).unwrap();
    let cookies = parse_cookie_values(cookies.to_str().unwrap());

    let cookie = cookies.get(JWT_COOKIE_NAME).unwrap();
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));

    let result = auth::validate_token(cookie, banned_token_store).await;
    assert!(result.is_err());

    app.delete_database(&app.db_name.clone()).await;
}
