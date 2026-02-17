use auth_service::utils::constants::JWT_COOKIE_NAME;

use crate::helpers::{TestApp, get_random_email, get_all_cookies};

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

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
    let cookies = get_all_cookies(&response);

    assert_eq!(response.status().as_u16(), 200);

    // Verify token
    let verify_token_body = serde_json::json!({
        "token": cookies.get(JWT_COOKIE_NAME).unwrap(),
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let verify_token_body = serde_json::json!({
        "token": "token1234",
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let app = TestApp::new().await;

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
    let cookies = get_all_cookies(&response);

    assert_eq!(response.status().as_u16(), 200);

    // Logout
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    // Verify that the token is banned

    let verify_token_body = serde_json::json!({
        "token": cookies.get(JWT_COOKIE_NAME).unwrap(),
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "token": 12346,
        }),
        serde_json::json!({
            "token": true,
        }),
        serde_json::json!({
            "token2": "tokenval",
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}