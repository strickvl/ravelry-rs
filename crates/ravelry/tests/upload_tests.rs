//! Integration tests for the upload API.
//!
//! Key test: POST /upload/image.json should NOT include Authorization header.

mod common;

use ravelry::types::UploadFile;
use wiremock::matchers::{method, path, header_exists};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_request_token_requires_auth() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/upload/request_token.json"))
        .and(header_exists("authorization"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "upload_token": "test_token_123"
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let response = client.upload().request_token().await.unwrap();

    assert_eq!(response.upload_token, "test_token_123");
}

#[tokio::test]
async fn test_upload_image_no_auth_header() {
    let server = MockServer::start().await;

    // This mock explicitly checks that there is NO authorization header
    // The upload/image endpoint is unauthenticated per Ravelry API docs
    Mock::given(method("POST"))
        .and(path("/upload/image.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "uploads": [
                        {"file0": {"image_id": 12345}}
                    ]
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let file = UploadFile::new("test.jpg", vec![1, 2, 3, 4]);

    let response = client
        .upload()
        .image("test_token", vec![file])
        .await
        .unwrap();

    assert_eq!(response.uploads.len(), 1);
    let first = &response.uploads[0];
    assert!(first.contains_key("file0"));
    assert_eq!(first["file0"].image_id, 12345);
}

#[tokio::test]
async fn test_upload_rejects_more_than_10_files() {
    let server = MockServer::start().await;
    let client = common::test_client(&server);

    // Create 11 files
    let files: Vec<UploadFile> = (0..11)
        .map(|i| UploadFile::new(format!("file{}.jpg", i), vec![i as u8]))
        .collect();

    let result = client.upload().image("test_token", files).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Maximum 10 files"));
}

#[tokio::test]
async fn test_upload_rejects_empty_files() {
    let server = MockServer::start().await;
    let client = common::test_client(&server);

    let result = client
        .upload()
        .image("test_token", Vec::<UploadFile>::new())
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("At least one file"));
}

#[tokio::test]
async fn test_image_status_no_auth() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/upload/image/status.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "uploads": [
                        {"file0": {"image_id": 99}}
                    ]
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let response = client
        .upload()
        .image_status("test_token")
        .await
        .unwrap();

    assert_eq!(response.uploads.len(), 1);
}
