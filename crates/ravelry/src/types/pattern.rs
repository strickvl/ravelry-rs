//! Pattern types for the Ravelry API.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;
use super::photo::PhotoSmall;

/// Pattern information returned in search results and lists.
///
/// This is a minimal representation suitable for displaying in lists.
/// Use `PatternFull` for complete pattern details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PatternList {
    /// Unique pattern ID.
    pub id: u64,

    /// Pattern name.
    pub name: String,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// The first/primary photo for this pattern.
    #[serde(default)]
    pub first_photo: Option<PhotoSmall>,

    /// The pattern designer's name.
    #[serde(default)]
    pub designer_name: Option<String>,

    /// Whether the pattern is free.
    #[serde(default)]
    pub free: Option<bool>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full pattern details returned when fetching a single pattern.
///
/// This contains all available fields for a pattern. The API returns
/// many more fields than are explicitly modeled here; use the `extra`
/// field to access them.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PatternFull {
    /// Unique pattern ID.
    pub id: u64,

    /// Pattern name.
    pub name: String,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// The first/primary photo for this pattern.
    #[serde(default)]
    pub first_photo: Option<PhotoSmall>,

    /// The pattern designer's name.
    #[serde(default)]
    pub designer_name: Option<String>,

    /// Whether the pattern is free.
    #[serde(default)]
    pub free: Option<bool>,

    /// Pattern notes in HTML format.
    #[serde(default)]
    pub notes_html: Option<String>,

    /// Pattern notes as plain text.
    #[serde(default)]
    pub notes: Option<String>,

    /// Number of projects made from this pattern.
    #[serde(default)]
    pub projects_count: Option<u64>,

    /// Number of users who queued this pattern.
    #[serde(default)]
    pub queued_projects_count: Option<u64>,

    /// Number of users who favorited this pattern.
    #[serde(default)]
    pub favorites_count: Option<u64>,

    /// Number of comments on this pattern.
    #[serde(default)]
    pub comments_count: Option<u64>,

    /// Average rating (0-5 scale).
    #[serde(default)]
    pub rating_average: Option<f64>,

    /// Number of ratings.
    #[serde(default)]
    pub rating_count: Option<u64>,

    /// Difficulty rating average.
    #[serde(default)]
    pub difficulty_average: Option<f64>,

    /// Number of difficulty ratings.
    #[serde(default)]
    pub difficulty_count: Option<u64>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
