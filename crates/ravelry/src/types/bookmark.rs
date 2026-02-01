//! Bookmark (favorite) types for the Ravelry API.
//!
//! Bookmarks are favorites that can reference various entity types
//! (patterns, yarns, projects, etc.).

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

/// Bookmark information returned in list responses.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookmarkList {
    /// Unique bookmark ID.
    pub id: u64,

    /// The type of favorited item (e.g., "pattern", "yarn").
    #[serde(rename = "type", default)]
    pub type_name: Option<String>,

    /// The ID of the favorited item.
    #[serde(default)]
    pub favorited_id: Option<u64>,

    /// User comment on the bookmark.
    #[serde(default)]
    pub comment: Option<String>,

    /// When the bookmark was created.
    #[serde(default)]
    pub created_at: Option<String>,

    /// Tags applied to this bookmark.
    #[serde(default)]
    pub tag_names: Option<Vec<String>>,

    /// The favorited item (polymorphic based on type).
    #[serde(default)]
    pub favorited: Option<serde_json::Value>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full bookmark details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BookmarkFull {
    /// Unique bookmark ID.
    pub id: u64,

    /// The type of favorited item.
    #[serde(rename = "type", default)]
    pub type_name: Option<String>,

    /// The ID of the favorited item.
    #[serde(default)]
    pub favorited_id: Option<u64>,

    /// User comment on the bookmark.
    #[serde(default)]
    pub comment: Option<String>,

    /// When the bookmark was created.
    #[serde(default)]
    pub created_at: Option<String>,

    /// Tags applied to this bookmark.
    #[serde(default)]
    pub tag_names: Option<Vec<String>>,

    /// The favorited item (polymorphic based on type).
    #[serde(default)]
    pub favorited: Option<serde_json::Value>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Bookmark data for creating or updating.
#[derive(Serialize, Debug, Default, Clone)]
pub struct BookmarkPost {
    /// The type of item to favorite.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    /// The ID of the item to favorite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorited_id: Option<u64>,

    /// Comment on the bookmark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Tags for the bookmark (comma-separated or as array).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<String>,

    /// Capture any additional fields for flexibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl BookmarkPost {
    /// Create a new empty bookmark post.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the type of item to favorite.
    pub fn type_name(mut self, type_name: impl Into<String>) -> Self {
        self.type_name = Some(type_name.into());
        self
    }

    /// Set the ID of the item to favorite.
    pub fn favorited_id(mut self, id: u64) -> Self {
        self.favorited_id = Some(id);
        self
    }

    /// Set the comment.
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Set the tags.
    pub fn tag_names(mut self, tags: impl Into<String>) -> Self {
        self.tag_names = Some(tags.into());
        self
    }
}
