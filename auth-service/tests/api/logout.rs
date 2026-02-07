use auth_service::{utils::constants::JWT_COOKIE_NAME, utils::auth, domain::ErrorResponse};

use reqwest::{Url, cookie::CookieStore};

use crate::helpers::{TestApp, parse_cookie_values};

#[tokio::test]
async fn should_return_400_if_there_are_no_cookies() {
    let app = TestApp::new().await;

    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    let cookies = app.cookie_jar.cookies(url);
    
    assert!(cookies.is_none())
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let url = &Url::parse("http://127.0.0.1").expect("Failed to parse URL");
    // add a cookie
    app.cookie_jar.add_cookie_str(
        "random=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
        url,
    );
    
    let cookies = app.cookie_jar.cookies(url).unwrap();
    let cookies = parse_cookie_values(cookies.to_str().unwrap());

    let cookie_exists = cookies.contains_key(JWT_COOKIE_NAME);
    
    assert!(!cookie_exists)
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

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

    let result = auth::validate_token(cookie).await;
    assert!(result.is_err());

}