use crate::helpers::{TestApp, get_random_email};

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