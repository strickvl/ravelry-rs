//! Integration tests for error handling.

mod common;

use ravelry::RavelryError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_rate_limited_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/current_user.json"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "60")
                .set_body_json(serde_json::json!({
                    "error": "Rate limited"
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let result = client.root().current_user().await;

    assert!(result.is_err());
    let err = result.unwrap_err();

    match err {
        RavelryError::RateLimited { retry_after, .. } => {
            assert_eq!(retry_after, Some(std::time::Duration::from_secs(60)));
        }
        other => panic!("Expected RateLimited, got {:?}", other),
    }
}

#[tokio::test]
async fn test_not_modified_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/current_user.json"))
        .respond_with(ResponseTemplate::new(304).insert_header("etag", "\"abc123\""))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let result = client.root().current_user().await;

    assert!(result.is_err());
    let err = result.unwrap_err();

    match err {
        RavelryError::NotModified { etag } => {
            assert_eq!(etag, Some("\"abc123\"".to_string()));
        }
        other => panic!("Expected NotModified, got {:?}", other),
    }
}

#[tokio::test]
async fn test_api_error_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/patterns/99999.json"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "error": "Pattern not found"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let result = client.patterns().show(99999).await;

    assert!(result.is_err());
    let err = result.unwrap_err();

    match err {
        RavelryError::ApiStatus { status, body } => {
            assert_eq!(status.as_u16(), 404);
            assert_eq!(body["error"], "Pattern not found");
        }
        other => panic!("Expected ApiStatus, got {:?}", other),
    }
}

#[tokio::test]
async fn test_error_is_retryable() {
    let rate_limited = RavelryError::RateLimited {
        retry_after: Some(std::time::Duration::from_secs(30)),
        body: None,
    };
    assert!(rate_limited.is_retryable());
    assert_eq!(
        rate_limited.retry_after(),
        Some(std::time::Duration::from_secs(30))
    );

    let not_modified = RavelryError::NotModified { etag: None };
    assert!(!not_modified.is_retryable());
}
