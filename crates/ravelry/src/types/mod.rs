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

pub mod bookmark;
pub mod bundle;
pub mod common;
pub mod friend;
pub mod message;
pub mod pattern;
pub mod photo;
pub mod project;
pub mod stash;
pub mod upload;
pub mod user;
pub mod yarn;

pub use bookmark::*;
pub use bundle::*;
pub use common::*;
pub use friend::*;
pub use message::*;
pub use pattern::*;
pub use photo::*;
pub use project::*;
pub use stash::*;
pub use upload::*;
pub use user::*;
pub use yarn::*;
