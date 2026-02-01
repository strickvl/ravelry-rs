//! Favorites (bookmarks) API endpoints.
//!
//! Favorites allow users to bookmark patterns, yarns, projects, and other items.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::{PageParams, Paginator};
use crate::types::{BookmarkFull, BookmarkList, BookmarkPost};

/// Service for favorites-related API endpoints.
pub struct FavoritesApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> FavoritesApi<'a> {
    /// List a user's favorites.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::favorites::FavoritesListParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = FavoritesListParams::new().page_size(10);
    /// let response = client.favorites().list("username", &params).await?;
    /// for fav in response.favorites {
    ///     println!("{}: {:?}", fav.id, fav.type_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        username: &str,
        params: &FavoritesListParams,
    ) -> Result<FavoritesListResponse, RavelryError> {
        let path = format!("people/{}/favorites/list.json", username);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }

    /// Get details for a specific favorite.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.favorites().show("username", 12345).await?;
    /// println!("Favorite: {:?}", response.favorite);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(
        &self,
        username: &str,
        id: u64,
    ) -> Result<FavoritesShowResponse, RavelryError> {
        let path = format!("people/{}/favorites/{}.json", username, id);
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Create a new favorite.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::BookmarkPost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let bookmark = BookmarkPost::new()
    ///     .type_name("pattern")
    ///     .favorited_id(123456)
    ///     .comment("Love this pattern!");
    ///
    /// let response = client.favorites().create("username", &bookmark).await?;
    /// println!("Created favorite: {}", response.favorite.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        data: &BookmarkPost,
    ) -> Result<FavoritesMutateResponse, RavelryError> {
        let path = format!("people/{}/favorites/create.json", username);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Update an existing favorite.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::BookmarkPost;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let update = BookmarkPost::new().comment("Updated comment!");
    /// let response = client.favorites().update("username", 12345, &update).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        username: &str,
        id: u64,
        data: &BookmarkPost,
    ) -> Result<FavoritesMutateResponse, RavelryError> {
        let path = format!("people/{}/favorites/{}.json", username, id);
        let req = self.client.post_data(&path, data);
        self.client.send_json(req).await
    }

    /// Delete a favorite.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.favorites().delete("username", 12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(
        &self,
        username: &str,
        id: u64,
    ) -> Result<FavoritesMutateResponse, RavelryError> {
        let path = format!("people/{}/favorites/{}.json", username, id);
        let req = self.client.delete(&path);
        self.client.send_json(req).await
    }

    /// Add a favorite to a bundle.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.favorites().add_to_bundle("username", 12345, 67890).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_to_bundle(
        &self,
        username: &str,
        favorite_id: u64,
        bundle_id: u64,
    ) -> Result<FavoritesMutateResponse, RavelryError> {
        let path = format!(
            "people/{}/favorites/{}/add_to_bundle.json",
            username, favorite_id
        );
        let req = self
            .client
            .post(&path)
            .query(&[("bundle_id", bundle_id.to_string())]);
        self.client.send_json(req).await
    }

    /// Remove a favorite from a bundle.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.favorites().remove_from_bundle("username", 12345, 67890).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_from_bundle(
        &self,
        username: &str,
        favorite_id: u64,
        bundle_id: u64,
    ) -> Result<FavoritesMutateResponse, RavelryError> {
        let path = format!(
            "people/{}/favorites/{}/remove_from_bundle.json",
            username, favorite_id
        );
        let req = self
            .client
            .post(&path)
            .query(&[("bundle_id", bundle_id.to_string())]);
        self.client.send_json(req).await
    }
}

/// Parameters for listing favorites.
#[derive(Serialize, Default, Debug, Clone)]
pub struct FavoritesListParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Filter by type (e.g., "pattern", "yarn").
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<String>,

    /// Filter by tag name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Query string to search favorites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl FavoritesListParams {
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

    /// Filter by item type.
    pub fn type_filter(mut self, type_name: impl Into<String>) -> Self {
        self.type_filter = Some(type_name.into());
        self
    }

    /// Filter by tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Set the sort order.
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    /// Set the search query.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
}

/// Response from listing favorites.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FavoritesListResponse {
    /// The list of favorites.
    pub favorites: Vec<BookmarkList>,

    /// Pagination information.
    #[serde(default)]
    pub paginator: Option<Paginator>,
}

/// Response from showing a single favorite.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FavoritesShowResponse {
    /// The favorite details.
    pub favorite: BookmarkFull,
}

/// Response from mutating a favorite (create/update/delete/bundle ops).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FavoritesMutateResponse {
    /// The mutated favorite.
    pub favorite: BookmarkFull,
}
