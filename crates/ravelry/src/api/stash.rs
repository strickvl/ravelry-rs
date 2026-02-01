//! Stash-related API endpoints.
//!
//! Stash represents yarn that users own or have in their collection.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::PageParams;
use crate::types::{StashFull, StashPost, StashSmall};

/// Service for stash-related API endpoints.
pub struct StashApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> StashApi<'a> {
    /// List a user's stash.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::stash::StashListParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = StashListParams::new().page_size(10);
    /// let response = client.stash().list("username", &params).await?;
    /// for entry in response.stash {
    ///     println!("{}: {:?}", entry.id, entry.yarn_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        username: &str,
        params: &StashListParams,
    ) -> Result<StashListResponse, RavelryError> {
        let path = format!("people/{}/stash/list.json", username);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }

    /// Get details for a specific stash entry.
    ///
    /// The `id` can be either a numeric ID or a permalink string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.stash().show("username", "1").await?;
    /// println!("Stash: {:?}", response.stash.yarn_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(&self, username: &str, id: &str) -> Result<StashShowResponse, RavelryError> {
        let path = format!("people/{}/stash/{}.json", username, id);
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Create a new stash entry.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::StashPost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let stash = StashPost::new()
    ///     .yarn_id(573)
    ///     .colorway_name("Ocean Blue")
    ///     .skeins(3.0);
    ///
    /// let response = client.stash().create("username", &stash).await?;
    /// println!("Created stash: {}", response.stash.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        data: &StashPost,
    ) -> Result<StashCreateResponse, RavelryError> {
        let path = format!("people/{}/stash/create.json", username);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Update an existing stash entry.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::StashPost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let update = StashPost::new().skeins(2.0);
    /// let response = client.stash().update("username", 123, &update).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        username: &str,
        id: u64,
        data: &StashPost,
    ) -> Result<StashUpdateResponse, RavelryError> {
        let path = format!("people/{}/stash/{}.json", username, id);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Delete a stash entry.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.stash().delete("username", 123).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        username: &str,
        id: u64,
    ) -> Result<StashDeleteResponse, RavelryError> {
        let path = format!("people/{}/stash/{}.json", username, id);
        let req = self.client.delete(&path);
        self.client.send_json(req).await
    }
}

/// Parameters for listing stash.
#[derive(Serialize, Default, Debug, Clone)]
pub struct StashListParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Sort order (e.g., "recent", "alpha", "weight", "colorfamily", "yards").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl StashListParams {
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

/// Response from listing stash.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashListResponse {
    /// The list of stash entries.
    pub stash: Vec<StashSmall>,
}

/// Response from showing a single stash entry.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashShowResponse {
    /// The stash entry details.
    pub stash: StashFull,
}

/// Response from creating a stash entry.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashCreateResponse {
    /// The created stash entry.
    pub stash: StashFull,
}

/// Response from updating a stash entry.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashUpdateResponse {
    /// The updated stash entry.
    pub stash: StashFull,
}

/// Response from deleting a stash entry.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashDeleteResponse {
    /// The deleted stash entry.
    pub stash: StashFull,
}
