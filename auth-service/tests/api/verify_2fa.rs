use auth_service::domain::{LoginAttemptId, TwoFACode};

use crate::helpers::{TestApp, get_random_email};

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