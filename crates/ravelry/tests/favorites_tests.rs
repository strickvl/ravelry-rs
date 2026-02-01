//! Integration tests for the favorites API.

mod common;

use ravelry::api::favorites::FavoritesListParams;
use ravelry::types::BookmarkPost;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_favorites() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/people/testuser/favorites/list.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "favorites": [
                        {
                            "id": 1,
                            "type": "pattern",
                            "favorited_id": 12345,
                            "comment": "Love this!"
                        }
                    ],
                    "paginator": {
                        "page": 1,
                        "page_count": 1,
                        "page_size": 10,
                        "results": 1,
                        "last_page": 1
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let params = FavoritesListParams::new().page_size(10);
    let response = client.favorites().list("testuser", &params).await.unwrap();

    assert_eq!(response.favorites.len(), 1);
    assert_eq!(response.favorites[0].id, 1);
    assert_eq!(response.favorites[0].type_name, Some("pattern".to_string()));
}

#[tokio::test]
async fn test_create_favorite() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/people/testuser/favorites/create.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "favorite": {
                        "id": 999,
                        "type": "pattern",
                        "favorited_id": 12345
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let post = BookmarkPost::new()
        .type_name("pattern")
        .favorited_id(12345)
        .comment("Great pattern!");

    let response = client.favorites().create("testuser", &post).await.unwrap();
    assert_eq!(response.favorite.id, 999);
}

#[tokio::test]
async fn test_add_favorite_to_bundle() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/people/testuser/favorites/100/add_to_bundle.json"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "favorite": {
                        "id": 100,
                        "type": "pattern"
                    }
                })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::test_client(&server);
    let response = client
        .favorites()
        .add_to_bundle("testuser", 100, 200)
        .await
        .unwrap();
    assert_eq!(response.favorite.id, 100);
}
