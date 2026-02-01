//! The main Ravelry API client.

use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::{
    bundles::BundlesApi, bundled_items::BundledItemsApi, favorites::FavoritesApi,
    friends::FriendsApi, messages::MessagesApi, patterns::PatternsApi, projects::ProjectsApi,
    root::RootApi, stash::StashApi, upload::UploadApi, yarns::YarnsApi,
};
use crate::auth::{AuthKind, Authenticator, NoAuth};
use crate::error::{map_error_response, RavelryError};
use crate::request_options::RequestOptions;

/// Controls whether authentication is applied to a request.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum AuthMode {
    /// Apply the configured authenticator (default behavior).
    #[default]
    Default,
    /// Do not apply authentication (for unauthenticated endpoints like upload/image).
    None,
}

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

    /// Access yarn-related endpoints.
    pub fn yarns(&self) -> YarnsApi<'_> {
        YarnsApi { client: self }
    }

    /// Access project-related endpoints.
    pub fn projects(&self) -> ProjectsApi<'_> {
        ProjectsApi { client: self }
    }

    /// Access stash-related endpoints.
    pub fn stash(&self) -> StashApi<'_> {
        StashApi { client: self }
    }

    /// Access message-related endpoints.
    pub fn messages(&self) -> MessagesApi<'_> {
        MessagesApi { client: self }
    }

    /// Access upload-related endpoints.
    pub fn upload(&self) -> UploadApi<'_> {
        UploadApi { client: self }
    }

    /// Access favorites-related endpoints.
    pub fn favorites(&self) -> FavoritesApi<'_> {
        FavoritesApi { client: self }
    }

    /// Access bundles-related endpoints.
    pub fn bundles(&self) -> BundlesApi<'_> {
        BundlesApi { client: self }
    }

    /// Access bundled items endpoints.
    pub fn bundled_items(&self) -> BundledItemsApi<'_> {
        BundledItemsApi { client: self }
    }

    /// Access friends-related endpoints.
    pub fn friends(&self) -> FriendsApi<'_> {
        FriendsApi { client: self }
    }

    // --- Internal Request Helpers ---

    /// Create a GET request for the given path.
    pub(crate) fn get(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::GET, path)
    }

    /// Create a POST request for the given path.
    pub(crate) fn post(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::POST, path)
    }

    /// Create a DELETE request for the given path.
    pub(crate) fn delete(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::DELETE, path)
    }

    /// Create a PUT request for the given path.
    #[allow(dead_code)]
    pub(crate) fn put(&self, path: &str) -> RequestBuilder {
        self.request(reqwest::Method::PUT, path)
    }

    /// Create a GET request without authentication.
    ///
    /// Used for endpoints like upload status that don't require auth.
    pub(crate) fn get_no_auth(&self, path: &str) -> RequestBuilder {
        self.request_with_auth(reqwest::Method::GET, path, AuthMode::None)
    }

    /// Create a POST request without authentication.
    ///
    /// Used for endpoints like upload/image that explicitly don't use auth.
    pub(crate) fn post_no_auth(&self, path: &str) -> RequestBuilder {
        self.request_with_auth(reqwest::Method::POST, path, AuthMode::None)
    }

    /// Create a POST request with the common `{ "data": ... }` wrapper.
    ///
    /// Many Ravelry endpoints expect mutations to be wrapped in a `data` field.
    pub(crate) fn post_data<T: serde::Serialize>(&self, path: &str, data: &T) -> RequestBuilder {
        #[derive(serde::Serialize)]
        struct Wrapper<'a, T: serde::Serialize> {
            data: &'a T,
        }

        self.post(path).json(&Wrapper { data })
    }

    /// Create a request for the given method and path (with default auth).
    fn request(&self, method: reqwest::Method, path: &str) -> RequestBuilder {
        self.request_with_auth(method, path, AuthMode::Default)
    }

    /// Create a request with explicit auth mode control.
    fn request_with_auth(
        &self,
        method: reqwest::Method,
        path: &str,
        auth_mode: AuthMode,
    ) -> RequestBuilder {
        let url = self.base_url.join(path).expect("Invalid path");
        let mut req = self.http.request(method, url);

        // Apply authentication only if requested
        if matches!(auth_mode, AuthMode::Default) {
            req = self.auth.apply(req);
        }

        // Apply default options
        if self.defaults.debug {
            req = req.query(&[("debug", "1")]);
        }

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

    /// Send a request that returns an empty response (for DELETE, mark_read, etc.).
    #[allow(dead_code)]
    pub(crate) async fn send_empty(&self, req: RequestBuilder) -> Result<(), RavelryError> {
        let resp = req.send().await?;

        if resp.status().is_success() {
            Ok(())
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
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        Ok(RavelryClient {
            http,
            base_url: self.base_url,
            auth: self.auth,
            defaults: self.defaults,
        })
    }
}
