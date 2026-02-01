//! Common type utilities and helpers.

use std::collections::HashMap;

/// Type alias for capturing unknown JSON fields.
///
/// This is used with `#[serde(flatten)]` to preserve any fields
/// not explicitly defined in our structs.
pub type ExtraFields = HashMap<String, serde_json::Value>;
