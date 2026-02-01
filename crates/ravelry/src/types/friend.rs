//! Friend/friendship types for the Ravelry API.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;
use super::user::UserSmall;

/// Friendship information.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Friendship {
    /// Unique friendship ID.
    pub id: u64,

    /// The friend user.
    #[serde(default)]
    pub friend: Option<UserSmall>,

    /// Whether this is a mutual friendship.
    #[serde(default)]
    pub mutual: Option<bool>,

    /// When the friendship was created.
    #[serde(default)]
    pub created_at: Option<String>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Activity item from a friend's feed.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FriendActivity {
    /// Activity ID.
    #[serde(default)]
    pub id: Option<u64>,

    /// Type of activity (e.g., "project_added", "pattern_favorited").
    #[serde(rename = "type", default)]
    pub activity_type: Option<String>,

    /// When the activity occurred.
    #[serde(default)]
    pub created_at: Option<String>,

    /// The user who performed the activity.
    #[serde(default)]
    pub user: Option<UserSmall>,

    /// Activity-specific data (polymorphic).
    #[serde(default)]
    pub data: Option<serde_json::Value>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
