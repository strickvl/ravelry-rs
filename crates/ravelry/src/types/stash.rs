//! Stash types for the Ravelry API.
//!
//! Stash represents yarn that a user owns or has in their collection.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;
use super::photo::PhotoSmall;

/// Stash entry information returned in lists.
///
/// This is a minimal representation suitable for displaying in lists.
/// Use [`StashFull`] for complete stash details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashSmall {
    /// Unique stash entry ID.
    pub id: u64,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// Optional custom name for this stash entry.
    #[serde(default)]
    pub name: Option<String>,

    /// The yarn ID if linked to a yarn in the database.
    #[serde(default)]
    pub yarn_id: Option<u64>,

    /// The yarn name.
    #[serde(default)]
    pub yarn_name: Option<String>,

    /// The colorway/color name.
    #[serde(default)]
    pub colorway_name: Option<String>,

    /// The first/primary photo for this stash entry.
    #[serde(default)]
    pub first_photo: Option<PhotoSmall>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full stash entry information returned when fetching a single entry.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StashFull {
    /// Unique stash entry ID.
    pub id: u64,

    /// URL-friendly unique identifier.
    pub permalink: String,

    /// Optional custom name for this stash entry.
    #[serde(default)]
    pub name: Option<String>,

    /// The yarn ID if linked to a yarn in the database.
    #[serde(default)]
    pub yarn_id: Option<u64>,

    /// The yarn name.
    #[serde(default)]
    pub yarn_name: Option<String>,

    /// The colorway/color name.
    #[serde(default)]
    pub colorway_name: Option<String>,

    /// Dye lot identifier.
    #[serde(default)]
    pub dye_lot: Option<String>,

    /// Total skeins/units.
    #[serde(default)]
    pub skeins: Option<f64>,

    /// Notes about this stash entry.
    #[serde(default)]
    pub notes: Option<String>,

    /// Notes as HTML.
    #[serde(default)]
    pub notes_html: Option<String>,

    /// Where this yarn was acquired.
    #[serde(default)]
    pub location: Option<String>,

    /// When it was acquired.
    #[serde(default)]
    pub acquired: Option<String>,

    /// Personal rating (1-5).
    #[serde(default)]
    pub personal_rating: Option<u32>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Stash data for creating or updating a stash entry.
#[derive(Serialize, Debug, Default, Clone)]
pub struct StashPost {
    /// Link to a yarn in the database.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yarn_id: Option<u64>,

    /// Custom name for this stash entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Colorway/color name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colorway_name: Option<String>,

    /// Dye lot identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dye_lot: Option<String>,

    /// Number of skeins/units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeins: Option<f64>,

    /// Notes about this stash entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Where it was acquired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Personal rating (1-5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_rating: Option<u32>,

    /// Capture any additional fields for flexibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl StashPost {
    /// Create a new empty stash post.
    pub fn new() -> Self {
        Self::default()
    }

    /// Link to a yarn.
    pub fn yarn_id(mut self, id: u64) -> Self {
        self.yarn_id = Some(id);
        self
    }

    /// Set the colorway name.
    pub fn colorway_name(mut self, name: impl Into<String>) -> Self {
        self.colorway_name = Some(name.into());
        self
    }

    /// Set the dye lot.
    pub fn dye_lot(mut self, lot: impl Into<String>) -> Self {
        self.dye_lot = Some(lot.into());
        self
    }

    /// Set the number of skeins.
    pub fn skeins(mut self, count: f64) -> Self {
        self.skeins = Some(count);
        self
    }
}
