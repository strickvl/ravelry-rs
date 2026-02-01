//! Yarn types for the Ravelry API.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;
use super::photo::PhotoSmall;

/// Yarn information returned in search results and lists.
///
/// This is a minimal representation suitable for displaying in lists.
/// Use [`YarnFull`] for complete yarn details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct YarnList {
    /// Unique yarn ID.
    pub id: u64,

    /// Yarn name.
    pub name: String,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// The yarn company/brand name.
    #[serde(default)]
    pub yarn_company_name: Option<String>,

    /// The first/primary photo for this yarn.
    #[serde(default)]
    pub first_photo: Option<PhotoSmall>,

    /// Average rating (1-5).
    #[serde(default)]
    pub rating_average: Option<f64>,

    /// Number of ratings.
    #[serde(default)]
    pub rating_count: Option<u64>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full yarn information returned when fetching a single yarn.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct YarnFull {
    /// Unique yarn ID.
    pub id: u64,

    /// Yarn name.
    pub name: String,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// The yarn company/brand name.
    #[serde(default)]
    pub yarn_company_name: Option<String>,

    /// The yarn company ID.
    #[serde(default)]
    pub yarn_company_id: Option<u64>,

    /// Average rating (1-5).
    #[serde(default)]
    pub rating_average: Option<f64>,

    /// Number of ratings.
    #[serde(default)]
    pub rating_count: Option<u64>,

    /// Number of projects using this yarn.
    #[serde(default)]
    pub projects_count: Option<u64>,

    /// Number of stash entries for this yarn.
    #[serde(default)]
    pub stashes_count: Option<u64>,

    /// Yarn weight category (e.g., "Fingering", "DK", "Worsted").
    #[serde(default)]
    pub yarn_weight_name: Option<String>,

    /// Fiber content (e.g., "100% Merino Wool").
    #[serde(default)]
    pub fiber_content: Option<String>,

    /// Whether this yarn is discontinued.
    #[serde(default)]
    pub discontinued: Option<bool>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
