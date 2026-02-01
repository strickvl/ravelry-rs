//! # Ravelry
//!
//! A typed, async Rust client for the [Ravelry API](https://www.ravelry.com/api).
//!
//! ## Quick Start
//!
//! ```no_run
//! use ravelry::{RavelryClient, auth::BasicAuth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), ravelry::RavelryError> {
//!     let auth = BasicAuth::new("your_access_key", "your_personal_key");
//!     let client = RavelryClient::builder(auth).build()?;
//!
//!     let user = client.root().current_user().await?;
//!     println!("Logged in as: {}", user.user.username);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod pagination;
pub mod request_options;
pub mod types;

// Re-export main entry points for ergonomic usage
pub use client::{RavelryClient, RavelryClientBuilder};
pub use error::RavelryError;
pub use pagination::{PageParams, Paginator};

// Re-export auth types
pub use auth::{AuthKind, Authenticator, BasicAuth, OAuth2Auth, OAuth2Token, RavelryOAuth2Client};
