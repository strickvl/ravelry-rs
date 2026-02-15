//! Pagination types for Ravelry API requests and responses.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Maximum retries per page when hitting rate limits.
const MAX_RETRIES_PER_PAGE: u32 = 5;

/// Default backoff when the Retry-After header is missing.
const DEFAULT_BACKOFF: Duration = Duration::from_secs(5);

/// Polite delay between consecutive page fetches (to avoid hammering the API).
const INTER_PAGE_DELAY: Duration = Duration::from_millis(500);

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

/// Progress info passed to the callback after each successful page fetch.
#[derive(Debug, Clone)]
pub struct PageProgress {
    /// The page just fetched (1-indexed).
    pub current_page: u32,
    /// Total pages available (from the paginator).
    pub total_pages: u32,
    /// Total items collected so far.
    pub items_so_far: usize,
    /// Total results reported by the API.
    pub total_results: u32,
}

/// Collect all pages from a paginated endpoint.
///
/// Automatically retries on rate-limit (429) errors using the `Retry-After`
/// header, with exponential backoff. Up to 5 retries per page.
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
    collect_all_pages_with_progress(initial_page_size, max_pages, fetch, |_| {}).await
}

/// Like [`collect_all_pages`], but calls `on_progress` after each successful page.
///
/// This is useful for CLIs that want to show progress (e.g., "Fetching page 5/930...").
///
/// # Example
///
/// ```no_run
/// # use ravelry::{RavelryClient, auth::BasicAuth, PageParams, RavelryError};
/// # use ravelry::pagination::collect_all_pages_with_progress;
/// use ravelry::api::patterns::PatternSearchParams;
///
/// # async fn example() -> Result<(), RavelryError> {
/// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
/// let all_patterns = collect_all_pages_with_progress(50, None, |page_params| {
///     let client = &client;
///     let params = PatternSearchParams {
///         query: Some("baby blanket".to_string()),
///         page: page_params,
///         ..Default::default()
///     };
///     async move {
///         let resp = client.patterns().search(&params).await?;
///         Ok((resp.patterns, resp.paginator))
///     }
/// }, |progress| {
///     eprintln!("Page {}/{} ({} items)", progress.current_page, progress.total_pages, progress.items_so_far);
/// }).await?;
/// # Ok(())
/// # }
/// ```
pub async fn collect_all_pages_with_progress<T, F, Fut, P>(
    initial_page_size: u32,
    max_pages: Option<u32>,
    fetch: F,
    on_progress: P,
) -> Result<Vec<T>, crate::RavelryError>
where
    F: Fn(PageParams) -> Fut,
    Fut: std::future::Future<Output = Result<(Vec<T>, Paginator), crate::RavelryError>>,
    P: Fn(&PageProgress),
{
    let mut all_items = Vec::new();
    let mut current_page = 1u32;
    let mut pages_fetched = 0u32;

    loop {
        if let Some(max) = max_pages {
            if pages_fetched >= max {
                break;
            }
        }

        let page_params = PageParams {
            page: Some(current_page),
            page_size: Some(initial_page_size),
        };

        // Fetch with retry on rate-limit errors
        let mut retries = 0u32;
        let (items, paginator) = loop {
            match fetch(page_params.clone()).await {
                Ok(result) => break result,
                Err(e) if e.is_retryable() && retries < MAX_RETRIES_PER_PAGE => {
                    let base_wait = e.retry_after().unwrap_or(DEFAULT_BACKOFF);
                    // Scale wait by retry count for exponential backoff
                    let wait = base_wait * (retries + 1);
                    tokio::time::sleep(wait).await;
                    retries += 1;
                }
                Err(e) => return Err(e),
            }
        };

        all_items.extend(items);
        pages_fetched += 1;

        on_progress(&PageProgress {
            current_page,
            total_pages: paginator.last_page,
            items_so_far: all_items.len(),
            total_results: paginator.results,
        });

        if paginator.has_next() {
            current_page = paginator.page + 1;
            // Be a good API citizen: wait between requests
            tokio::time::sleep(INTER_PAGE_DELAY).await;
        } else {
            break;
        }
    }

    Ok(all_items)
}
