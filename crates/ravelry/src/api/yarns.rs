//! Yarn-related API endpoints.
//!
//! Yarns are the materials used for knitting and crochet projects.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::{PageParams, Paginator};
use crate::types::{YarnFull, YarnList};

/// Service for yarn-related API endpoints.
pub struct YarnsApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> YarnsApi<'a> {
    /// Search for yarns.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::yarns::YarnSearchParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = YarnSearchParams::new()
    ///     .query("merino")
    ///     .page_size(10);
    ///
    /// let response = client.yarns().search(&params).await?;
    /// for yarn in response.yarns {
    ///     println!("{}: {}", yarn.id, yarn.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(
        &self,
        params: &YarnSearchParams,
    ) -> Result<YarnsSearchResponse, RavelryError> {
        let req = self.client.get("yarns/search.json").query(params);
        self.client.send_json(req).await
    }

    /// Get details for a specific yarn.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.yarns().show(573, &Default::default()).await?;
    /// println!("Yarn: {}", response.yarn.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(
        &self,
        id: u64,
        params: &YarnShowParams,
    ) -> Result<YarnShowResponse, RavelryError> {
        let path = format!("yarns/{}.json", id);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }
}

/// Parameters for yarn search.
#[derive(Serialize, Default, Debug, Clone)]
pub struct YarnSearchParams {
    /// Free-text search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Sort order (e.g., "best", "rating", "projects").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Include personal attributes in the response (requires auth).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_attributes: Option<bool>,
}

impl YarnSearchParams {
    /// Create new search params with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
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

/// Response from yarn search.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct YarnsSearchResponse {
    /// The list of yarns matching the search.
    pub yarns: Vec<YarnList>,

    /// Pagination information.
    pub paginator: Paginator,
}

/// Parameters for showing a single yarn.
#[derive(Serialize, Default, Debug, Clone)]
pub struct YarnShowParams {
    /// Extra parts to include (space delimited: "colorways", "availability").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

impl YarnShowParams {
    /// Create new show params with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Include extra parts in the response.
    pub fn include(mut self, parts: impl Into<String>) -> Self {
        self.include = Some(parts.into());
        self
    }
}

/// Response from showing a single yarn.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct YarnShowResponse {
    /// The yarn details.
    pub yarn: YarnFull,
}
