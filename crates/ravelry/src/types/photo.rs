//! Photo types for the Ravelry API.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

/// Small photo information (typically used in lists and thumbnails).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotoSmall {
    /// Unique photo ID.
    pub id: u64,

    /// URL to the thumbnail version.
    #[serde(default)]
    pub thumbnail_url: Option<String>,

    /// URL to the small version.
    #[serde(default)]
    pub small_url: Option<String>,

    /// URL to the square version (cropped).
    #[serde(default)]
    pub square_url: Option<String>,

    /// URL to the medium version.
    #[serde(default)]
    pub medium_url: Option<String>,

    /// Sort order (lower numbers appear first).
    #[serde(default)]
    pub sort_order: Option<i32>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
