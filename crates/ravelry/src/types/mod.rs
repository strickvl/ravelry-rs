//! Data types for Ravelry API entities.
//!
//! The Ravelry API returns different field sets depending on context:
//!
//! - **List types** (e.g., `PatternList`): Minimal fields returned in search results
//! - **Full types** (e.g., `PatternFull`): Complete fields when fetching a single entity
//! - **Post types** (e.g., `PatternPost`): Writable fields for create/update operations
//!
//! All types use `#[serde(flatten)]` with a HashMap to capture unknown fields,
//! making them resilient to API changes.

pub mod common;
pub mod pattern;
pub mod photo;
pub mod user;

pub use common::*;
pub use pattern::*;
pub use photo::*;
pub use user::*;
