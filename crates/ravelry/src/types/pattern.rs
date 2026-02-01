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
