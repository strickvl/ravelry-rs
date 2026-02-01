//! Pattern-related API endpoints.
//!
//! Patterns are knitting/crochet instructions for creating items.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::{PageParams, Paginator};
use crate::types::{PatternFull, PatternList, ProjectSmall};

/// Service for pattern-related API endpoints.
pub struct PatternsApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> PatternsApi<'a> {
    /// Search for patterns.
    ///
    /// This is one of the most commonly used endpoints. You can search by
    /// query string and filter by various criteria.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::patterns::PatternSearchParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = PatternSearchParams::new()
    ///     .query("baby blanket")
    ///     .page_size(10);
    ///
    /// let response = client.patterns().search(&params).await?;
    /// for pattern in response.patterns {
    ///     println!("{}: {}", pattern.id, pattern.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(
        &self,
        params: &PatternSearchParams,
    ) -> Result<PatternsSearchResponse, RavelryError> {
        let req = self.client.get("patterns/search.json").query(params);
        self.client.send_json(req).await
    }

    /// Get details for a single pattern.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.patterns().show(123456).await?;
    /// println!("{}: {}", response.pattern.id, response.pattern.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(&self, id: u64) -> Result<PatternShowResponse, RavelryError> {
        let path = format!("patterns/{id}.json");
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Get projects made from a pattern.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::patterns::PatternProjectsParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = PatternProjectsParams::new().page_size(10);
    /// let response = client.patterns().projects(123456, &params).await?;
    /// for project in response.projects {
    ///     println!("{}: {}", project.id, project.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn projects(
        &self,
        id: u64,
        params: &PatternProjectsParams,
    ) -> Result<PatternProjectsResponse, RavelryError> {
        let path = format!("patterns/{id}/projects.json");
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }
}

/// Parameters for pattern search.
///
/// Use the builder methods to construct search parameters.
#[derive(Serialize, Default, Debug, Clone)]
pub struct PatternSearchParams {
    /// Free-text search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Include personal attributes in the response (requires auth).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_attributes: Option<bool>,

    /// Filter by craft type (e.g., "knitting", "crochet").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub craft: Option<String>,

    /// Sort order (e.g., "best_match", "recently_popular", "date").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl PatternSearchParams {
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

    /// Include personal attributes (requires authentication).
    pub fn personal_attributes(mut self, include: bool) -> Self {
        self.personal_attributes = Some(include);
        self
    }

    /// Filter by craft type (e.g., "knitting", "crochet").
    pub fn craft(mut self, craft: impl Into<String>) -> Self {
        self.craft = Some(craft.into());
        self
    }

    /// Set the sort order.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }
}

/// Response from pattern search.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PatternsSearchResponse {
    /// The list of patterns matching the search.
    pub patterns: Vec<PatternList>,

    /// Pagination information.
    pub paginator: Paginator,
}

/// Response from fetching a single pattern.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PatternShowResponse {
    /// The full pattern details.
    pub pattern: PatternFull,
}

/// Parameters for fetching projects made from a pattern.
#[derive(Serialize, Default, Debug, Clone)]
pub struct PatternProjectsParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Sort order (e.g., "completed", "started", "best").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl PatternProjectsParams {
    /// Create new params with defaults.
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

/// Response from fetching projects for a pattern.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PatternProjectsResponse {
    /// The list of projects made from this pattern.
    pub projects: Vec<ProjectSmall>,

    /// Pagination information.
    pub paginator: Paginator,
}
