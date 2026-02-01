//! Root-level API endpoints.
//!
//! These are endpoints that don't belong to a specific resource category.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::types::UserFull;

/// Service for root-level API endpoints.
pub struct RootApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> RootApi<'a> {
    /// Get the current authenticated user.
    ///
    /// This endpoint requires authentication and returns information about
    /// the user whose credentials are being used.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.root().current_user().await?;
    /// println!("Hello, {}!", response.user.username);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn current_user(&self) -> Result<CurrentUserResponse, RavelryError> {
        let req = self.client.get("current_user.json");
        self.client.send_json(req).await
    }
}

/// Response from `GET /current_user.json`.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CurrentUserResponse {
    /// The authenticated user's information.
    pub user: UserFull,
}
