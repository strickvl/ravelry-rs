//! Common test utilities for wiremock-based integration tests.

use ravelry::{auth::BasicAuth, RavelryClient};
use wiremock::MockServer;

/// Create a test client configured to use the mock server.
pub fn test_client(server: &MockServer) -> RavelryClient {
    let base_url = server.uri().parse().expect("Invalid mock server URL");
    RavelryClient::builder(BasicAuth::new("test_user", "test_key"))
        .base_url(base_url)
        .build()
        .expect("Failed to build test client")
}

/// Create an unauthenticated test client.
#[allow(dead_code)]
pub fn test_client_no_auth(server: &MockServer) -> RavelryClient {
    use ravelry::auth::NoAuth;
    let base_url = server.uri().parse().expect("Invalid mock server URL");
    RavelryClient::builder(NoAuth)
        .base_url(base_url)
        .build()
        .expect("Failed to build test client")
}
