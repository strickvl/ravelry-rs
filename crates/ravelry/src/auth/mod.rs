//! Authentication strategies for the Ravelry API.
//!
//! The Ravelry API supports multiple authentication methods:
//!
//! - **Basic Auth (Read-only)**: Access key + secret key for unauthenticated endpoints only
//! - **Basic Auth (Personal)**: Access key + personal key for full access to personal data
//! - **OAuth2**: For third-party applications with user authorization
//!
//! # Choosing an Authentication Method
//!
//! | Method | Use Case | Tokens Expire? |
//! |--------|----------|----------------|
//! | Basic (Read-only) | Public data only, no user context | No |
//! | Basic (Personal) | Personal scripts, full access to your account | No |
//! | OAuth2 | Third-party apps, accessing other users' data | Yes (24h) |

mod basic;
mod oauth2;

pub use basic::BasicAuth;
pub use oauth2::{OAuth2Auth, OAuth2Token, RavelryOAuth2Client};
use reqwest::RequestBuilder;

/// The type of authentication being used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthKind {
    /// No authentication
    None,
    /// HTTP Basic authentication
    Basic,
    /// OAuth2 bearer token (future)
    OAuth2,
}

/// Trait for authentication strategies.
///
/// Implementors can modify outgoing requests to add authentication credentials.
pub trait Authenticator: Send + Sync {
    /// Apply authentication to a request builder.
    fn apply(&self, req: RequestBuilder) -> RequestBuilder;

    /// Return the kind of authentication this provides.
    fn kind(&self) -> AuthKind;
}

/// A no-op authenticator for unauthenticated requests.
#[derive(Debug, Clone, Default)]
pub struct NoAuth;

impl Authenticator for NoAuth {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder {
        req
    }

    fn kind(&self) -> AuthKind {
        AuthKind::None
    }
}
