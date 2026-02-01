//! OAuth2 authentication for Ravelry API.
//!
//! This module provides:
//! - [`OAuth2Auth`]: An [`Authenticator`] that adds bearer tokens to requests
//! - [`OAuth2Token`]: A serializable token for storage/refresh
//! - [`RavelryOAuth2Client`]: Helper for OAuth2 authorization flows
//!
//! # OAuth2 Flow Overview
//!
//! 1. Create an [`RavelryOAuth2Client`] with your client credentials
//! 2. Generate an authorization URL and open it in the user's browser
//! 3. User authorizes your app and is redirected back with a code
//! 4. Exchange the code for tokens using [`RavelryOAuth2Client::exchange_code`]
//! 5. Create an [`OAuth2Auth`] from the access token to make API calls
//! 6. When tokens expire, use [`RavelryOAuth2Client::refresh`] to get new ones
//!
//! # Example
//!
//! ```no_run
//! use ravelry::auth::{OAuth2Auth, OAuth2Token, RavelryOAuth2Client};
//! use ravelry::RavelryClient;
//!
//! # async fn example() -> Result<(), ravelry::RavelryError> {
//! // Create the OAuth2 client for the authorization flow
//! let oauth_client = RavelryOAuth2Client::new(
//!     "your_client_id",
//!     "your_client_secret",
//!     "https://localhost:8080/callback",
//! )?;
//!
//! // Generate authorization URL (user opens this in browser)
//! let (auth_url, csrf_state) = oauth_client.authorize_url(vec!["offline".to_string()]);
//! println!("Open this URL: {}", auth_url);
//!
//! // After user authorizes, exchange the code for tokens
//! let code = "code_from_callback";
//! let token = oauth_client.exchange_code(code).await?;
//!
//! // Use the token to make API calls
//! let auth = OAuth2Auth::new(&token.access_token);
//! let client = RavelryClient::builder(auth).build()?;
//! # Ok(())
//! # }
//! ```

use oauth2::{ClientId, ClientSecret, CsrfToken, RedirectUrl};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use time::OffsetDateTime;

use super::{AuthKind, Authenticator};
use crate::RavelryError;

/// Ravelry OAuth2 authorization URL.
pub const AUTH_URL: &str = "https://www.ravelry.com/oauth2/auth";

/// Ravelry OAuth2 token URL.
pub const TOKEN_URL: &str = "https://www.ravelry.com/oauth2/token";

// ─────────────────────────────────────────────────────────────────────────────
// OAuth2Auth - Authenticator implementation
// ─────────────────────────────────────────────────────────────────────────────

/// OAuth2 bearer token authentication.
///
/// This implements [`Authenticator`] and adds the access token as a
/// `Bearer` token in the `Authorization` header.
#[derive(Clone)]
pub struct OAuth2Auth {
    access_token: String,
}

impl OAuth2Auth {
    /// Create a new OAuth2 authenticator with the given access token.
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
        }
    }

    /// Returns the access token.
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

impl Authenticator for OAuth2Auth {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder {
        req.bearer_auth(&self.access_token)
    }

    fn kind(&self) -> AuthKind {
        AuthKind::OAuth2
    }
}

impl std::fmt::Debug for OAuth2Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuth2Auth")
            .field("access_token", &"[REDACTED]")
            .finish()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OAuth2Token - Serializable token for storage
// ─────────────────────────────────────────────────────────────────────────────

/// An OAuth2 token that can be serialized for storage.
///
/// This stores the access token, optional refresh token, and expiration time.
/// Use [`OAuth2Token::is_expired`] to check if the token needs refreshing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuth2Token {
    /// The access token for API requests.
    pub access_token: String,

    /// The refresh token for obtaining new access tokens.
    /// Only present if the `offline` scope was requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    /// When the access token expires (absolute time).
    /// Stored as absolute time for easier persistence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,

    /// The scopes granted by this token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// The token type (usually "Bearer").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

impl OAuth2Token {
    /// Check if the token is expired or will expire within the given skew duration.
    ///
    /// # Arguments
    ///
    /// * `skew` - Buffer time before actual expiration to consider the token expired.
    ///   Use this to refresh tokens proactively (e.g., 5 minutes before expiry).
    ///
    /// # Returns
    ///
    /// `true` if the token is expired or will expire within the skew duration,
    /// `false` if the token is still valid or has no expiration time.
    pub fn is_expired(&self, skew: Duration) -> bool {
        match self.expires_at {
            Some(expires_at) => {
                let now = OffsetDateTime::now_utc();
                let skew = time::Duration::try_from(skew).unwrap_or(time::Duration::ZERO);
                now + skew >= expires_at
            }
            None => false, // No expiration means it doesn't expire (treat as valid)
        }
    }

    /// Create an [`OAuth2Auth`] from this token.
    pub fn to_auth(&self) -> OAuth2Auth {
        OAuth2Auth::new(&self.access_token)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RawTokenResponse - Internal deserialization helper
// ─────────────────────────────────────────────────────────────────────────────

/// Raw OAuth2 token response from the server.
/// This is used internally to parse the JSON response before converting to [`OAuth2Token`].
#[derive(Deserialize)]
struct RawTokenResponse {
    access_token: String,
    token_type: Option<String>,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

impl RawTokenResponse {
    fn into_oauth2_token(self) -> OAuth2Token {
        let expires_at = self
            .expires_in
            .map(|secs| OffsetDateTime::now_utc() + time::Duration::seconds(secs as i64));

        OAuth2Token {
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            expires_at,
            scope: self.scope,
            token_type: self.token_type,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RavelryOAuth2Client - OAuth2 flow helper
// ─────────────────────────────────────────────────────────────────────────────

/// OAuth2 client for Ravelry authorization flows.
///
/// This wraps the oauth2 crate's client and provides methods for:
/// - Generating authorization URLs
/// - Exchanging authorization codes for tokens
/// - Refreshing expired tokens
///
/// # Important: Ravelry-specific configuration
///
/// Ravelry requires client credentials to be sent via HTTP Basic Auth
/// (in the Authorization header), not in the request body.
pub struct RavelryOAuth2Client {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
    http_client: reqwest::Client,
}

impl RavelryOAuth2Client {
    /// Create a new OAuth2 client for Ravelry.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Your Ravelry OAuth2 client ID
    /// * `client_secret` - Your Ravelry OAuth2 client secret
    /// * `redirect_uri` - The callback URL (e.g., `https://localhost:8080/callback`)
    ///
    /// # Errors
    ///
    /// Returns an error if the URLs are invalid or the HTTP client fails to build.
    pub fn new(
        client_id: &str,
        client_secret: &str,
        redirect_uri: &str,
    ) -> Result<Self, RavelryError> {
        let redirect_url = RedirectUrl::new(redirect_uri.to_string())
            .map_err(|e| RavelryError::Auth(format!("Invalid redirect URI: {e}")))?;

        let http_client = reqwest::Client::builder()
            .build()
            .map_err(|e| RavelryError::Auth(format!("Failed to build HTTP client: {e}")))?;

        Ok(Self {
            client_id: ClientId::new(client_id.to_string()),
            client_secret: ClientSecret::new(client_secret.to_string()),
            redirect_uri: redirect_url,
            http_client,
        })
    }

    /// Generate an authorization URL for the user to visit.
    ///
    /// # Arguments
    ///
    /// * `scopes` - The OAuth2 scopes to request (e.g., `["offline", "message-write"]`)
    ///
    /// # Returns
    ///
    /// A tuple of (authorization_url, csrf_state). The CSRF state should be
    /// verified when the user is redirected back to your callback URL.
    pub fn authorize_url(&self, scopes: impl IntoIterator<Item = String>) -> (url::Url, String) {
        let csrf_token = CsrfToken::new_random();
        let scope_str: Vec<String> = scopes.into_iter().collect();

        let mut url = url::Url::parse(AUTH_URL).expect("AUTH_URL is valid");
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("client_id", self.client_id.as_str());
            query.append_pair("redirect_uri", self.redirect_uri.as_str());
            query.append_pair("response_type", "code");
            query.append_pair("state", csrf_token.secret());
            if !scope_str.is_empty() {
                query.append_pair("scope", &scope_str.join(" "));
            }
        }

        (url, csrf_token.secret().clone())
    }

    /// Exchange an authorization code for tokens.
    ///
    /// # Arguments
    ///
    /// * `code` - The authorization code from the OAuth2 callback
    ///
    /// # Errors
    ///
    /// Returns an error if the token exchange fails.
    pub async fn exchange_code(&self, code: &str) -> Result<OAuth2Token, RavelryError> {
        let response = self
            .http_client
            .post(TOKEN_URL)
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.secret()))
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await
            .map_err(|e| RavelryError::Auth(format!("Token exchange request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RavelryError::Auth(format!(
                "Token exchange failed with {status}: {body}"
            )));
        }

        let token_response: RawTokenResponse = response
            .json()
            .await
            .map_err(|e| RavelryError::Auth(format!("Failed to parse token response: {e}")))?;

        Ok(token_response.into_oauth2_token())
    }

    /// Refresh an expired access token.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - The refresh token from a previous token response
    ///
    /// # Errors
    ///
    /// Returns an error if the refresh fails.
    pub async fn refresh(&self, refresh_token: &str) -> Result<OAuth2Token, RavelryError> {
        let response = self
            .http_client
            .post(TOKEN_URL)
            .basic_auth(self.client_id.as_str(), Some(self.client_secret.secret()))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
            ])
            .send()
            .await
            .map_err(|e| RavelryError::Auth(format!("Token refresh request failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RavelryError::Auth(format!(
                "Token refresh failed with {status}: {body}"
            )));
        }

        let token_response: RawTokenResponse = response
            .json()
            .await
            .map_err(|e| RavelryError::Auth(format!("Failed to parse token response: {e}")))?;

        Ok(token_response.into_oauth2_token())
    }
}

impl std::fmt::Debug for RavelryOAuth2Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RavelryOAuth2Client")
            .field("client", &"[OAuth2Client]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_token_not_expired() {
        let token = OAuth2Token {
            access_token: "test".to_string(),
            refresh_token: None,
            expires_at: Some(OffsetDateTime::now_utc() + time::Duration::hours(1)),
            scope: None,
            token_type: None,
        };
        assert!(!token.is_expired(Duration::from_secs(60)));
    }

    #[test]
    fn test_oauth2_token_expired() {
        let token = OAuth2Token {
            access_token: "test".to_string(),
            refresh_token: None,
            expires_at: Some(OffsetDateTime::now_utc() - time::Duration::hours(1)),
            scope: None,
            token_type: None,
        };
        assert!(token.is_expired(Duration::from_secs(0)));
    }

    #[test]
    fn test_oauth2_token_expires_within_skew() {
        let token = OAuth2Token {
            access_token: "test".to_string(),
            refresh_token: None,
            expires_at: Some(OffsetDateTime::now_utc() + time::Duration::minutes(4)),
            scope: None,
            token_type: None,
        };
        // Token expires in 4 minutes, but we have a 5 minute skew
        assert!(token.is_expired(Duration::from_secs(300)));
    }

    #[test]
    fn test_oauth2_token_no_expiry() {
        let token = OAuth2Token {
            access_token: "test".to_string(),
            refresh_token: None,
            expires_at: None,
            scope: None,
            token_type: None,
        };
        // No expiry means not expired
        assert!(!token.is_expired(Duration::from_secs(0)));
    }

    #[test]
    fn test_oauth2_auth_kind() {
        let auth = OAuth2Auth::new("test_token");
        assert_eq!(auth.kind(), AuthKind::OAuth2);
    }
}
