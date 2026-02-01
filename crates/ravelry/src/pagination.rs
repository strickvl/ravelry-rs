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

/// Collect all pages from a paginated endpoint.
///
/// This is a helper for CLI `--all` flags and similar use cases where you
/// want to fetch all results regardless of pagination.
///
/// # Arguments
///
/// * `initial_page_size` - Number of results per page (used for all requests)
/// * `max_pages` - Optional limit on the number of pages to fetch
/// * `fetch` - An async function that takes page params and returns (items, paginator)
///
/// # Example
///
/// ```no_run
/// # use ravelry::{RavelryClient, auth::BasicAuth, PageParams, RavelryError};
/// # use ravelry::pagination::collect_all_pages;
/// use ravelry::api::patterns::PatternSearchParams;
///
/// # async fn example() -> Result<(), RavelryError> {
/// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
/// let all_patterns = collect_all_pages(50, None, |page_params| {
///     let client = &client; // Borrow the client
///     let params = PatternSearchParams {
///         query: Some("baby blanket".to_string()),
///         page: page_params,
///         ..Default::default()
///     };
///     async move {
///         let resp = client.patterns().search(&params).await?;
///         Ok((resp.patterns, resp.paginator))
///     }
/// }).await?;
///
/// println!("Found {} patterns total", all_patterns.len());
/// # Ok(())
/// # }
/// ```
pub async fn collect_all_pages<T, F, Fut>(
    initial_page_size: u32,
    max_pages: Option<u32>,
    fetch: F,
) -> Result<Vec<T>, crate::RavelryError>
where
    F: Fn(PageParams) -> Fut,
    Fut: std::future::Future<Output = Result<(Vec<T>, Paginator), crate::RavelryError>>,
{
    let mut all_items = Vec::new();
    let mut current_page = 1u32;
    let mut pages_fetched = 0u32;

    loop {
        // Check if we've hit the max pages limit
        if let Some(max) = max_pages {
            if pages_fetched >= max {
                break;
            }
        }

        let page_params = PageParams {
            page: Some(current_page),
            page_size: Some(initial_page_size),
        };

        let (items, paginator) = fetch(page_params).await?;
        all_items.extend(items);
        pages_fetched += 1;

        // Check if there are more pages
        if paginator.has_next() {
            current_page = paginator.page + 1;
        } else {
            break;
        }
    }

    Ok(all_items)
}
