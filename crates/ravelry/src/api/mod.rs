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
//! # Ok(())
//! # }
//! ```

pub mod patterns;
pub mod root;
