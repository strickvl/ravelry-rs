//! Error types for the Ravelry API client.

use reqwest::StatusCode;
use std::time::Duration;

/// The main error type for Ravelry API operations.
#[derive(thiserror::Error, Debug)]
pub enum RavelryError {
    /// An HTTP transport error occurred.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned a non-success status code.
    #[error("API error {status}: {body}")]
    ApiStatus {
        /// The HTTP status code
        status: StatusCode,
        /// The response body (parsed as JSON if possible)
        body: serde_json::Value,
    },

    /// Rate limited by the API. Check `retry_after` for when to retry.
    #[error("Rate limited, retry after {retry_after:?}")]
    RateLimited {
        /// Duration to wait before retrying (from Retry-After header)
        retry_after: Option<Duration>,
        /// The response body, if any
        body: Option<serde_json::Value>,
    },

    /// Resource not modified (ETag matched).
    #[error("Not modified (ETag match)")]
    NotModified {
        /// The ETag value, if present
        etag: Option<String>,
    },

    /// Authentication failed.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// URL parsing error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// JSON deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid request parameters.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// I/O error (e.g., reading files for upload).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl RavelryError {
    /// Returns `true` if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            RavelryError::RateLimited { .. } => true,
            RavelryError::Http(e) if e.is_timeout() || e.is_connect() => true,
            _ => false,
        }
    }

    /// If rate limited, returns the duration to wait before retrying.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            RavelryError::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

/// Maps an HTTP response to a `RavelryError`.
///
/// This is used internally by the client to convert non-success responses.
pub(crate) async fn map_error_response(resp: reqwest::Response) -> RavelryError {
    let status = resp.status();

    // Handle 304 Not Modified
    if status == StatusCode::NOT_MODIFIED {
        let etag = resp
            .headers()
            .get("etag")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        return RavelryError::NotModified { etag };
    }

    // Handle 429 Rate Limited
    if status == StatusCode::TOO_MANY_REQUESTS {
        let retry_after = resp
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs);

        let body = resp.json::<serde_json::Value>().await.ok();

        return RavelryError::RateLimited { retry_after, body };
    }

    // Try to parse body as JSON, fallback to raw text
    let body = match resp.text().await {
        Ok(text) => {
            serde_json::from_str(&text).unwrap_or_else(|_| serde_json::json!({ "raw": text }))
        }
        Err(_) => serde_json::json!({ "error": "Failed to read response body" }),
    };

    RavelryError::ApiStatus { status, body }
}
