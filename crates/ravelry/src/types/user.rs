//! User types for the Ravelry API.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;

/// Full user information returned from `/current_user.json` and similar endpoints.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserFull {
    /// Unique user ID.
    pub id: u64,

    /// Username (unique identifier for the user).
    pub username: String,

    /// Display name (may be different from username).
    #[serde(default)]
    pub name: Option<String>,

    /// URL to the user's small avatar image.
    #[serde(default)]
    pub small_photo_url: Option<String>,

    /// URL to the user's tiny avatar image.
    #[serde(default)]
    pub tiny_photo_url: Option<String>,

    /// Capture any additional fields not explicitly defined.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Minimal user information returned in lists and references.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSmall {
    /// Unique user ID.
    pub id: u64,

    /// Username.
    pub username: String,

    /// URL to the user's tiny avatar image.
    #[serde(default)]
    pub tiny_photo_url: Option<String>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
