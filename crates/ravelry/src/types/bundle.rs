//! Bundle types for the Ravelry API.
//!
//! Bundles are user-created collections that can contain favorites/bookmarks.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

/// Bundle information returned in list responses.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundleList {
    /// Unique bundle ID.
    pub id: u64,

    /// Bundle name.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-friendly identifier.
    #[serde(default)]
    pub permalink: Option<String>,

    /// Number of items in the bundle.
    #[serde(default)]
    pub bundled_items_count: Option<u64>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full bundle details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundleFull {
    /// Unique bundle ID.
    pub id: u64,

    /// Bundle name.
    #[serde(default)]
    pub name: Option<String>,

    /// URL-friendly identifier.
    #[serde(default)]
    pub permalink: Option<String>,

    /// Number of items in the bundle.
    #[serde(default)]
    pub bundled_items_count: Option<u64>,

    /// Bundle description.
    #[serde(default)]
    pub notes: Option<String>,

    /// Whether the bundle is public.
    #[serde(default)]
    pub is_public: Option<bool>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Bundle data for creating or updating.
#[derive(Serialize, Debug, Default, Clone)]
pub struct BundlePost {
    /// Bundle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Bundle description/notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Whether the bundle is public.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    /// Capture any additional fields for flexibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl BundlePost {
    /// Create a new empty bundle post.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the bundle name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the bundle notes.
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    /// Set whether the bundle is public.
    pub fn is_public(mut self, public: bool) -> Self {
        self.is_public = Some(public);
        self
    }
}

/// Bundled item details (an item within a bundle).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BundledItemFull {
    /// Unique bundled item ID.
    pub id: u64,

    /// The bundle this item belongs to.
    #[serde(default)]
    pub bundle_id: Option<u64>,

    /// The bookmark/favorite this item references.
    #[serde(default)]
    pub favorite_id: Option<u64>,

    /// Position in the bundle.
    #[serde(default)]
    pub sort_order: Option<i32>,

    /// The actual item data (polymorphic).
    #[serde(default)]
    pub item: Option<serde_json::Value>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
