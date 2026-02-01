//! Integration tests for the messages API.

mod common;

use ravelry::api::messages::{MessageFolder, MessagesListParams};
use ravelry::types::MessagePost;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_messages() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/messages/list.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "messages": [
                        {
                            "id": 1,
                            "subject": "Hello",
                            "read_message": false
                        },
                        {
                            "id": 2,
                            "subject": "World",
                            "read_message": true
                        }
                    ],
                    "paginator": {
                        "page": 1,
                        "page_count": 1,
                        "page_size": 20,
                        "results": 2,
                        "last_page": 1
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let params = MessagesListParams::new().folder(MessageFolder::Inbox);
    let response = client.messages().list(&params).await.unwrap();

    assert_eq!(response.messages.len(), 2);
    assert_eq!(response.messages[0].subject, "Hello");
    assert_eq!(response.messages[1].subject, "World");
}

#[tokio::test]
async fn test_create_message() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/messages/create.json"))
        .and(body_json(serde_json::json!({
            "data": {
                "recipient_username": "testuser",
                "subject": "Test Subject",
                "content": "Test body"
            }
        })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "message": {
                        "id": 123,
                        "subject": "Test Subject"
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let post = MessagePost::new()
        .recipient_username("testuser")
        .subject("Test Subject")
        .content("Test body");

    let response = client.messages().create(&post).await.unwrap();
    assert_eq!(response.message.id, 123);
}

#[tokio::test]
async fn test_reply_to_message() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/messages/456/reply.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "message": {
                        "id": 789,
                        "subject": "Re: Original"
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let reply = MessagePost::new().content("Thanks for your message!");

    let response = client.messages().reply(456, &reply).await.unwrap();
    assert_eq!(response.message.id, 789);
}

#[tokio::test]
async fn test_unarchive_message() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/messages/123/unarchive.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "message": {
                        "id": 123,
                        "subject": "Unarchived message"
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let response = client.messages().unarchive(123).await.unwrap();
    assert_eq!(response.message.id, 123);
}
