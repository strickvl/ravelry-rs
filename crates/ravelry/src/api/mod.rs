//! API endpoint modules.
//!
//! Each module provides a "service" struct that groups related endpoints.
//! Access these through the corresponding methods on [`RavelryClient`].
//!
//! ```no_run
//! # use ravelry::{RavelryClient, auth::BasicAuth};
//! # async fn example() -> Result<(), ravelry::RavelryError> {
//! # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
//! // Access patterns endpoints
//! let search = client.patterns().search(&Default::default()).await?;
//!
//! // Access root endpoints
//! let me = client.root().current_user().await?;
//!
//! // Access yarns endpoints
//! let yarns = client.yarns().search(&Default::default()).await?;
//!
//! // Access projects endpoints
//! let projects = client.projects().list("username", &Default::default()).await?;
//! # Ok(())
//! # }
//! ```

pub mod bundled_items;
pub mod bundles;
pub mod favorites;
pub mod friends;
pub mod messages;
pub mod patterns;
pub mod projects;
pub mod root;
pub mod stash;
pub mod upload;
pub mod yarns;
