//! Project-related API endpoints.
//!
//! Projects are knitting/crochet items that users are working on or have completed.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::{PageParams, Paginator};
use crate::types::{ProjectFull, ProjectPost, ProjectSmall};

/// Service for project-related API endpoints.
pub struct ProjectsApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> ProjectsApi<'a> {
    /// List a user's projects.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::projects::ProjectsListParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = ProjectsListParams::new().page_size(10);
    /// let response = client.projects().list("username", &params).await?;
    /// for project in response.projects {
    ///     println!("{}: {}", project.id, project.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        username: &str,
        params: &ProjectsListParams,
    ) -> Result<ProjectsListResponse, RavelryError> {
        let path = format!("projects/{}/list.json", username);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }

    /// Get details for a specific project.
    ///
    /// The `id` can be either a numeric ID or a permalink string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.projects().show("username", "1", &Default::default()).await?;
    /// println!("Project: {}", response.project.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(
        &self,
        username: &str,
        id: &str,
        params: &ProjectShowParams,
    ) -> Result<ProjectShowResponse, RavelryError> {
        let path = format!("projects/{}/{}.json", username, id);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }

    /// Create a new project.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::ProjectPost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let project = ProjectPost::new()
    ///     .name("My New Sweater")
    ///     .pattern_id(12345);
    ///
    /// let response = client.projects().create("username", &project).await?;
    /// println!("Created project: {}", response.project.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        data: &ProjectPost,
    ) -> Result<ProjectCreateResponse, RavelryError> {
        let path = format!("projects/{}/create.json", username);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Update an existing project.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::ProjectPost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let update = ProjectPost::new().progress(50);
    /// let response = client.projects().update("username", 123, &update).await?;
    /// println!("Updated progress: {:?}", response.project.progress);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        username: &str,
        id: u64,
        data: &ProjectPost,
    ) -> Result<ProjectUpdateResponse, RavelryError> {
        let path = format!("projects/{}/{}.json", username, id);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Delete a project.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.projects().delete("username", 123).await?;
    /// println!("Deleted project: {}", response.project.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        username: &str,
        id: u64,
    ) -> Result<ProjectDeleteResponse, RavelryError> {
        let path = format!("projects/{}/{}.json", username, id);
        let req = self.client.delete(&path);
        self.client.send_json(req).await
    }
}

/// Parameters for listing projects.
#[derive(Serialize, Default, Debug, Clone)]
pub struct ProjectsListParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Sort order (e.g., "status", "name", "created", "started").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Extra parts to include (e.g., "collections").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

impl ProjectsListParams {
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

/// Response from listing projects.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectsListResponse {
    /// The list of projects.
    pub projects: Vec<ProjectSmall>,

    /// Pagination information.
    pub paginator: Paginator,
}

/// Parameters for showing a single project.
#[derive(Serialize, Default, Debug, Clone)]
pub struct ProjectShowParams {
    /// Extra parts to include (e.g., "comments").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

impl ProjectShowParams {
    /// Create new show params with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Include comments in the response.
    pub fn include_comments(mut self) -> Self {
        self.include = Some("comments".to_string());
        self
    }
}

/// Response from showing a single project.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectShowResponse {
    /// The project details.
    pub project: ProjectFull,
}

/// Response from creating a project.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectCreateResponse {
    /// The created project.
    pub project: ProjectFull,
}

/// Response from updating a project.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectUpdateResponse {
    /// The updated project.
    pub project: ProjectFull,
}

/// Response from deleting a project.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectDeleteResponse {
    /// The deleted project.
    pub project: ProjectFull,
}
