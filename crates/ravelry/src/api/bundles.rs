//! Bundles API endpoints.
//!
//! Bundles are user-created collections that can contain favorites.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::{PageParams, Paginator};
use crate::types::{BundleFull, BundleList, BundlePost};

/// Service for bundles-related API endpoints.
pub struct BundlesApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> BundlesApi<'a> {
    /// List a user's bundles.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::bundles::BundlesListParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = BundlesListParams::new().page_size(10);
    /// let response = client.bundles().list("username", &params).await?;
    /// for bundle in response.bundles {
    ///     println!("{}: {:?}", bundle.id, bundle.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        username: &str,
        params: &BundlesListParams,
    ) -> Result<BundlesListResponse, RavelryError> {
        let path = format!("people/{}/bundles/list.json", username);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }

    /// Get details for a specific bundle.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.bundles().show("username", 12345).await?;
    /// println!("Bundle: {:?}", response.bundle.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(&self, username: &str, id: u64) -> Result<BundleShowResponse, RavelryError> {
        let path = format!("people/{}/bundles/{}.json", username, id);
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Create a new bundle.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::BundlePost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let bundle = BundlePost::new()
    ///     .name("My Wishlist")
    ///     .is_public(true);
    ///
    /// let response = client.bundles().create("username", &bundle).await?;
    /// println!("Created bundle: {}", response.bundle.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        data: &BundlePost,
    ) -> Result<BundleMutateResponse, RavelryError> {
        let path = format!("people/{}/bundles/create.json", username);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Update an existing bundle.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::BundlePost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let update = BundlePost::new().name("Renamed Bundle");
    /// let response = client.bundles().update("username", 12345, &update).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        username: &str,
        id: u64,
        data: &BundlePost,
    ) -> Result<BundleMutateResponse, RavelryError> {
        let path = format!("people/{}/bundles/{}.json", username, id);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Delete a bundle.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.bundles().delete("username", 12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        username: &str,
        id: u64,
    ) -> Result<BundleMutateResponse, RavelryError> {
        let path = format!("people/{}/bundles/{}.json", username, id);
        let req = self.client.delete(&path);
        self.client.send_json(req).await
    }
}

/// Parameters for listing bundles.
#[derive(Serialize, Default, Debug, Clone)]
pub struct BundlesListParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl BundlesListParams {
    /// Create new list params with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page.page = Some(page);
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, size: u32) -> Self {
        self.page.page_size = Some(size);
        self
    }

    /// Set the sort order.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}

/// Response from listing bundles.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundlesListResponse {
    /// The list of bundles.
    pub bundles: Vec<BundleList>,

    /// Pagination information.
    #[serde(default)]
    pub paginator: Option<Paginator>,
}

/// Response from showing a single bundle.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundleShowResponse {
    /// The bundle details.
    pub bundle: BundleFull,
}

/// Response from mutating a bundle (create/update/delete).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundleMutateResponse {
    /// The mutated bundle.
    pub bundle: BundleFull,
}
