//! Request options for customizing API calls.

/// Options that can be applied to individual API requests.
#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    /// Include debug information in the response.
    ///
    /// When enabled, the API includes additional debugging info.
    pub debug: bool,

    /// ETag for conditional requests (If-None-Match header).
    ///
    /// If the resource hasn't changed, the API returns 304 Not Modified.
    pub if_none_match: Option<String>,
}

impl RequestOptions {
    /// Create new request options with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable debug mode.
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Set the If-None-Match header for conditional requests.
    pub fn if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.if_none_match = Some(etag.into());
        self
    }
}
