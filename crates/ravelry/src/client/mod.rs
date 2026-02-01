//! The main Ravelry API client.

use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::{patterns::PatternsApi, root::RootApi};
use crate::auth::{AuthKind, Authenticator, NoAuth};
use crate::error::{map_error_response, RavelryError};
use crate::request_options::RequestOptions;

/// The default Ravelry API base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.ravelry.com/";

/// A client for the Ravelry API.
///
/// Use [`RavelryClient::builder`] to create a new client with your authentication.
///
/// # Example
///
/// ```no_run
/// use ravelry::{RavelryClient, auth::BasicAuth};
///
/// # async fn example() -> Result<(), ravelry::RavelryError> {
/// let auth = BasicAuth::new("access_key", "personal_key");
/// let client = RavelryClient::builder(auth).build()?;
///
/// // Use the service pattern to access endpoints
/// let user = client.root().current_user().await?;
/// let patterns = client.patterns().search(&Default::default()).await?;
/// # Ok(())
/// # }
/// ```
pub struct RavelryClient {
    http: reqwest::Client,
    base_url: Url,
    auth: Box<dyn Authenticator + Send + Sync>,
    defaults: RequestOptions,
}

impl RavelryClient {
    /// Create a new builder with the given authenticator.
    pub fn builder<A>(auth: A) -> RavelryClientBuilder
    where
        A: Authenticator + 'static,
    {
        RavelryClientBuilder::new(auth)
    }

    /// Returns what kind of authentication this client is using.
    pub fn auth_kind(&self) -> AuthKind {
        self.auth.kind()
    }

    /// Returns the base URL for API requests.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    // --- Service Pattern Methods ---

    /// Access root-level endpoints (current_user, search, etc.).
    pub fn root(&self) -> RootApi<'_> {
        RootApi { client: self }
    }

    /// Access pattern-related endpoints.
    pub fn patterns(&self) -> PatternsApi<'_> {
        PatternsApi { client: self }
    }

    // --- Internal Request Helpers ---

    /// Create a GET request for the given path.
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        let url = self.base_url.join(path).expect("Invalid path");
        let mut req = self.http.get(url);

        // Apply authentication
        req = self.auth.apply(req);

        // Apply default options
        if let Some(ref etag) = self.defaults.if_none_match {
            req = req.header("If-None-Match", etag);
        }

        req
    }

    /// Send a request and deserialize the JSON response.
    pub(crate) async fn send_json<T: DeserializeOwned>(
        &self,
        req: RequestBuilder,
    ) -> Result<T, RavelryError> {
        let resp = req.send().await?;

        if resp.status().is_success() {
            Ok(resp.json().await?)
        } else {
            Err(map_error_response(resp).await)
        }
    }
}

impl std::fmt::Debug for RavelryClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RavelryClient")
            .field("base_url", &self.base_url)
            .field("auth_kind", &self.auth.kind())
            .finish_non_exhaustive()
    }
}

/// Builder for creating a [`RavelryClient`].
pub struct RavelryClientBuilder {
    base_url: Url,
    auth: Box<dyn Authenticator + Send + Sync>,
    defaults: RequestOptions,
}

impl RavelryClientBuilder {
    /// Create a new builder with the given authenticator.
    pub fn new<A>(auth: A) -> Self
    where
        A: Authenticator + 'static,
    {
        Self {
            base_url: Url::parse(DEFAULT_BASE_URL).expect("Invalid default URL"),
            auth: Box::new(auth),
            defaults: RequestOptions::default(),
        }
    }

    /// Create a new builder without authentication.
    ///
    /// This is useful for testing or for endpoints that don't require auth.
    pub fn unauthenticated() -> Self {
        Self::new(NoAuth)
    }

    /// Set a custom base URL.
    ///
    /// This is useful for testing against a mock server.
    pub fn base_url(mut self, url: Url) -> Self {
        self.base_url = url;
        self
    }

    /// Parse and set a custom base URL.
    pub fn base_url_str(mut self, url: &str) -> Result<Self, RavelryError> {
        self.base_url = Url::parse(url)?;
        Ok(self)
    }

    /// Enable debug mode for all requests.
    pub fn debug(mut self, enabled: bool) -> Self {
        self.defaults.debug = enabled;
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<RavelryClient, RavelryError> {
        let http = reqwest::Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(RavelryClient {
            http,
            base_url: self.base_url,
            auth: self.auth,
            defaults: self.defaults,
        })
    }
}
