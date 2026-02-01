//! Bundled items API endpoints.
//!
//! Bundled items are the individual items within a bundle.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::types::BundledItemFull;

/// Service for bundled items endpoints.
pub struct BundledItemsApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> BundledItemsApi<'a> {
    /// Get details for a specific bundled item.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.bundled_items().show(12345).await?;
    /// println!("Bundled item: {:?}", response.bundled_item);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(&self, id: u64) -> Result<BundledItemShowResponse, RavelryError> {
        let path = format!("bundled_items/{}.json", id);
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Delete a bundled item.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.bundled_items().delete(12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: u64) -> Result<BundledItemDeleteResponse, RavelryError> {
        let path = format!("bundled_items/{}.json", id);
        let req = self.client.delete(&path);
        self.client.send_json(req).await
    }
}

/// Response from showing a bundled item.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundledItemShowResponse {
    /// The bundled item details.
    pub bundled_item: BundledItemFull,

    /// The actual item (polymorphic).
    #[serde(default)]
    pub item: Option<serde_json::Value>,
}

/// Response from deleting a bundled item.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundledItemDeleteResponse {
    /// The deleted bundled item.
    pub bundled_item: BundledItemFull,
}
