//! HTTP Basic authentication for Ravelry API.
//!
//! Two modes are supported (both use the same HTTP mechanism):
//!
//! - **Read-only**: Access key + secret key → can only call unauthenticated endpoints
//! - **Personal**: Access key + personal key → full access for personal projects

use reqwest::RequestBuilder;

use super::{AuthKind, Authenticator};

/// HTTP Basic authentication credentials.
///
/// # Example
///
/// ```
/// use ravelry::auth::BasicAuth;
///
/// // Read-only access
/// let read_only = BasicAuth::new("access_key", "secret_key");
///
/// // Personal access (full access to your own data)
/// let personal = BasicAuth::new("access_key", "personal_key");
/// ```
#[derive(Clone)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    /// Create a new Basic auth with the given credentials.
    ///
    /// - `username`: Your Ravelry API access key
    /// - `password`: Either your secret key (read-only) or personal key (full access)
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Returns the username (access key).
    pub fn username(&self) -> &str {
        &self.username
    }
}

impl Authenticator for BasicAuth {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder {
        req.basic_auth(&self.username, Some(&self.password))
    }

    fn kind(&self) -> AuthKind {
        AuthKind::Basic
    }
}

impl std::fmt::Debug for BasicAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicAuth")
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .finish()
    }
}
