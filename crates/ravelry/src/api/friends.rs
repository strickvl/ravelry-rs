//! Friends API endpoints.
//!
//! Friends allow users to follow other users and see their activity.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::PageParams;
use crate::types::{FriendActivity, Friendship};

/// Service for friends-related API endpoints.
pub struct FriendsApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> FriendsApi<'a> {
    /// Get a user's friend activity feed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::friends::FriendsActivityParams;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = FriendsActivityParams::new().page_size(20);
    /// let response = client.friends().activity("username", &params).await?;
    /// for item in response.activity {
    ///     println!("{:?}: {:?}", item.activity_type, item.user);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn activity(
        &self,
        username: &str,
        params: &FriendsActivityParams,
    ) -> Result<FriendsActivityResponse, RavelryError> {
        let path = format!("people/{}/friends/activity.json", username);
        let req = self.client.get(&path).query(params);
        self.client.send_json(req).await
    }

    /// List a user's friends.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.friends().list("username").await?;
    /// for friendship in response.friendships {
    ///     println!("{}: {:?}", friendship.id, friendship.friend);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, username: &str) -> Result<FriendsListResponse, RavelryError> {
        let path = format!("people/{}/friends/list.json", username);
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Add a friend (follow a user).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// // friend_user_id is the ID of the user to follow
    /// let response = client.friends().create("myusername", 12345).await?;
    /// println!("Created friendship: {}", response.friendship.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        friend_user_id: u64,
    ) -> Result<FriendshipMutateResponse, RavelryError> {
        let path = format!("people/{}/friends/create.json", username);
        let req = self
            .client
            .post(&path)
            .query(&[("friend_user_id", friend_user_id.to_string())]);
        self.client.send_json(req).await
    }

    /// Remove a friend (unfollow a user).
    ///
    /// Note: This uses POST, not DELETE.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// // friendship_id is the ID of the friendship to remove
    /// let response = client.friends().destroy("myusername", 67890).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn destroy(
        &self,
        username: &str,
        friendship_id: u64,
    ) -> Result<FriendshipMutateResponse, RavelryError> {
        let path = format!("people/{}/friends/{}/destroy.json", username, friendship_id);
        let req = self.client.post(&path);
        self.client.send_json(req).await
    }
}

/// Parameters for friend activity feed.
#[derive(Serialize, Default, Debug, Clone)]
pub struct FriendsActivityParams {
    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Filter by activity type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
}

impl FriendsActivityParams {
    /// Create new activity params with defaults.
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

    /// Filter by activity type.
    pub fn activity_type(mut self, activity_type: impl Into<String>) -> Self {
        self.activity_type = Some(activity_type.into());
        self
    }
}

/// Response from friend activity feed.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FriendsActivityResponse {
    /// Activity items.
    #[serde(default)]
    pub activity: Vec<FriendActivity>,
}

/// Response from listing friends.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FriendsListResponse {
    /// List of friendships.
    #[serde(default)]
    pub friendships: Vec<Friendship>,

    /// Collections (if included).
    #[serde(default)]
    pub collections: Option<Vec<serde_json::Value>>,
}

/// Response from creating or destroying a friendship.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FriendshipMutateResponse {
    /// The friendship.
    pub friendship: Friendship,
}
