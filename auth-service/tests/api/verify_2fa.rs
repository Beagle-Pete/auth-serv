use auth_service::{domain::{Email, LoginAttemptId, TwoFACode}, utils::constants::JWT_COOKIE_NAME};

use crate::helpers::{TestApp, get_all_cookies, get_random_email};

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let email = Email::parse(random_email.clone()).unwrap();

    // Signup 
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // Verify 2FA
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let (login_attempt_id, two_fa_code) = two_fa_code_store.get_code(&email).await.unwrap();
    drop(two_fa_code_store);

    let verify_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": two_fa_code.as_ref(),
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Verify JWT token was created
    let cookies = get_all_cookies(&response);

    assert_eq!(response.status().as_u16(), 200);

    let verify_token_body = serde_json::json!({
        "token": cookies.get(JWT_COOKIE_NAME).unwrap(),
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let login_attempt_id = LoginAttemptId::default().as_ref().to_owned();
    let two_fa_code = TwoFACode::default().as_ref().to_owned();

    let test_cases = [
        serde_json::json!({
            "email": "testexample.com",
            "loginAttemptId": login_attempt_id,
            "2FACode": two_fa_code,
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": "some_id",
            "2FACode": two_fa_code,
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": "1234-asddf548-df",
            "2FACode": two_fa_code,
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": login_attempt_id,
            "2FACode": "12345",
        }),
        serde_json::json!({
            "email": get_random_email(),
            "loginAttemptId": login_attempt_id,
            "2FACode": "code",
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let email = Email::parse(random_email.clone()).unwrap();

    // Signup 
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // Verify 2FA
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let (login_attempt_id, two_fa_code) = two_fa_code_store.get_code(&email).await.unwrap();

    let verify_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": TwoFACode::default().as_ref(),
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);

    let verify_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": LoginAttemptId::default().as_ref(),
            "2FACode": two_fa_code.as_ref(),
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let email = Email::parse(random_email.clone()).unwrap();

    // Signup 
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // First Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // Store Login Attempt ID and 2FA Code
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let (login_attempt_id_one, two_fa_code_one) = two_fa_code_store.get_code(&email).await.unwrap();
    drop(two_fa_code_store);

    // Second Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // Verify 2FA
    let verify_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id_one.as_ref(),
            "2FACode": two_fa_code_one.as_ref(),
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {  
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let email = Email::parse(random_email.clone()).unwrap();

    // Signup 
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // Login
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // Verify 2FA
    let two_fa_code_store = app.two_fa_code_store.read().await;
    let (login_attempt_id, two_fa_code) = two_fa_code_store.get_code(&email).await.unwrap();
    drop(two_fa_code_store);

    let verify_body = serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": two_fa_code.as_ref(),
    });

    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // Verify JWT token was created
    let cookies = get_all_cookies(&response);

    assert_eq!(response.status().as_u16(), 200);

    let verify_token_body = serde_json::json!({
        "token": cookies.get(JWT_COOKIE_NAME).unwrap(),
    });

    let response = app.post_verify_token(&verify_token_body).await;

    assert_eq!(response.status().as_u16(), 200);  

    // Verify 2FA again
    let response = app.post_verify_2fa(&verify_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "loginAttemptId": "someid",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "loginAttemptId": "someid",
        }),
        serde_json::json!({
            "email": "test@example.com",
            "loginAttemptId": "someid",
            "2FACode": 123456
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}