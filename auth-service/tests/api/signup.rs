use auth_service::domain::ErrorResponse;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(), 201, "Failed for input: {:?}", test_case);
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": "exampleatexample.com",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "pass",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", test_case);

        assert_eq!(
            response.json::<ErrorResponse>()
                .await
                .expect("Could not deserialized response body to ErrorResponse")
                .error, 
            "Invalid credentials".to_owned()
        )
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases {
        app.post_signup(&test_case).await;
        let response = app.post_signup(&test_case).await;
        
        assert_eq!(response.status().as_u16(), 409, "Failed for input: {:?}", test_case);

        assert_eq!(
            response.json::<ErrorResponse>()
                .await
                .expect("Could not deserialized response body to ErrorResponse")
                .error, 
            "User already exists".to_owned()
        )
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}