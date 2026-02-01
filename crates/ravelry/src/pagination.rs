//! Pagination types for Ravelry API requests and responses.

use serde::{Deserialize, Serialize};

/// Parameters for paginated requests.
///
/// # Example
///
/// ```
/// use ravelry::PageParams;
///
/// let params = PageParams::default()
///     .page(2)
///     .page_size(25);
/// ```
#[derive(Serialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct PageParams {
    /// The page number (1-indexed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

impl PageParams {
    /// Create new page params with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }
}

/// Pagination metadata from API responses.
///
/// This is included in paginated responses and tells you about the total
/// number of results and pages available.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Paginator {
    /// Total number of pages available.
    pub page_count: u32,

    /// Current page number (1-indexed).
    pub page: u32,

    /// Number of results per page.
    pub page_size: u32,

    /// Total number of results across all pages.
    pub results: u32,

    /// The last page number (same as page_count).
    pub last_page: u32,
}

impl Paginator {
    /// Returns `true` if there are more pages after the current one.
    pub fn has_next(&self) -> bool {
        self.page < self.last_page
    }

    /// Returns `true` if there are pages before the current one.
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    /// Returns the next page number, if available.
    pub fn next_page(&self) -> Option<u32> {
        if self.has_next() {
            Some(self.page + 1)
        } else {
            None
        }
    }
}
