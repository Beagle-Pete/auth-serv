use auth_service::domain::ErrorResponse;

use crate::helpers::{TestApp, get_random_email};

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "exampleattest.com".to_owned(),
            "password": "password123".to_owned(),
        }),
        serde_json::json!({
            "email": "example@test.com".to_owned(),
            "password": "pass".to_owned(),
        }),
        serde_json::json!({
            "email": "exampleattest.com".to_owned(),
            "password": "pass".to_owned(),
        }),
    ];

    for test_case in test_cases {

        let response = app.post_login(&test_case).await;
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
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    
    // Do I need to create a new user here?
    let user_to_add = serde_json::json!({
            "email": "example@test.com",
            "password": "password123",
            "requires2FA": true
        });
    app.post_signup(&user_to_add).await;

    let test_cases = [
        serde_json::json!({
            "email": "example@test.com".to_owned(),
            "password": "password12".to_owned(),
        }),
        serde_json::json!({
            "email": "nonexistentuser@test.com".to_owned(),
            "password": "password123".to_owned(),
        }),
    ];

    for test_case in test_cases {

        let response = app.post_login(&test_case).await;
        assert_eq!(response.status().as_u16(), 401, "Failed for input: {:?}", test_case);

        assert_eq!(
            response.json::<ErrorResponse>()
                .await
                .expect("Could not deserialized response body to ErrorResponse")
                .error, 
            "Incorrect credentials".to_owned()
        )
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "user": get_random_email()
        }),
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}