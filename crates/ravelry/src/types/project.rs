//! Project types for the Ravelry API.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;
use super::photo::PhotoSmall;

/// Project information returned in search results and lists.
///
/// This is a minimal representation suitable for displaying in lists.
/// Use [`ProjectFull`] for complete project details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectSmall {
    /// Unique project ID.
    pub id: u64,

    /// Project name.
    pub name: String,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// The pattern ID if this project is linked to a pattern.
    #[serde(default)]
    pub pattern_id: Option<u64>,

    /// The pattern name if linked to a pattern.
    #[serde(default)]
    pub pattern_name: Option<String>,

    /// Current status ID.
    #[serde(default)]
    pub status_id: Option<u64>,

    /// Current status name.
    #[serde(default)]
    pub status_name: Option<String>,

    /// The first/primary photo for this project.
    #[serde(default)]
    pub first_photo: Option<PhotoSmall>,

    /// Progress percentage (0-100).
    #[serde(default)]
    pub progress: Option<u32>,

    /// When the project was started.
    #[serde(default)]
    pub started: Option<String>,

    /// When the project was completed.
    #[serde(default)]
    pub completed: Option<String>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full project information returned when fetching a single project.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectFull {
    /// Unique project ID.
    pub id: u64,

    /// Project name.
    pub name: String,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// The pattern ID if this project is linked to a pattern.
    #[serde(default)]
    pub pattern_id: Option<u64>,

    /// The pattern name if linked to a pattern.
    #[serde(default)]
    pub pattern_name: Option<String>,

    /// Current status ID.
    #[serde(default)]
    pub status_id: Option<u64>,

    /// Current status name.
    #[serde(default)]
    pub status_name: Option<String>,

    /// Progress percentage (0-100).
    #[serde(default)]
    pub progress: Option<u32>,

    /// When the project was started.
    #[serde(default)]
    pub started: Option<String>,

    /// When the project was completed.
    #[serde(default)]
    pub completed: Option<String>,

    /// Project notes (markdown/text).
    #[serde(default)]
    pub notes: Option<String>,

    /// Project notes as HTML.
    #[serde(default)]
    pub notes_html: Option<String>,

    /// Happiness rating (1-4).
    #[serde(default)]
    pub rating: Option<u32>,

    /// Number of favorites.
    #[serde(default)]
    pub favorites_count: Option<u64>,

    /// Number of comments.
    #[serde(default)]
    pub comments_count: Option<u64>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Project data for creating or updating a project.
#[derive(Serialize, Debug, Default, Clone)]
pub struct ProjectPost {
    /// Project name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Link to a pattern by ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_id: Option<u64>,

    /// Project status ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<u64>,

    /// Progress percentage (0-100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u32>,

    /// When the project was started (YYYY-MM-DD or similar).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,

    /// When the project was completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,

    /// Project notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Happiness rating (1-4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<u32>,

    /// Craft ID (knitting, crochet, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub craft_id: Option<u64>,

    /// Capture any additional fields for flexibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl ProjectPost {
    /// Create a new empty project post.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the project name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Link to a pattern.
    pub fn pattern_id(mut self, id: u64) -> Self {
        self.pattern_id = Some(id);
        self
    }

    /// Set the status.
    pub fn status_id(mut self, id: u64) -> Self {
        self.status_id = Some(id);
        self
    }

    /// Set the progress percentage.
    pub fn progress(mut self, pct: u32) -> Self {
        self.progress = Some(pct);
        self
    }
}
