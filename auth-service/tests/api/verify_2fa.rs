use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "loginAttempId": "someid",
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "loginAttempId": "someid",
        }),
        serde_json::json!({
            "email": "test@example.com",
            "loginAttempId": "someid",
            "2FACode": 123456
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}