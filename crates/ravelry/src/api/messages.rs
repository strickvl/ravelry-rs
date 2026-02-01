//! Message-related API endpoints.
//!
//! Private messages between Ravelry users.

use serde::{Deserialize, Serialize};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::pagination::{PageParams, Paginator};
use crate::types::{MessageFull, MessageList};

/// Service for message-related API endpoints.
pub struct MessagesApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> MessagesApi<'a> {
    /// List messages in a folder.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::api::messages::{MessagesListParams, MessageFolder};
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let params = MessagesListParams::new()
    ///     .folder(MessageFolder::Inbox)
    ///     .unread_only(true)
    ///     .page_size(20);
    ///
    /// let response = client.messages().list(&params).await?;
    /// for message in response.messages {
    ///     println!("{}: {}", message.id, message.subject);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        params: &MessagesListParams,
    ) -> Result<MessagesListResponse, RavelryError> {
        let req = self.client.get("messages/list.json").query(params);
        self.client.send_json(req).await
    }

    /// Get details for a specific message.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.messages().show(12345).await?;
    /// println!("Subject: {}", response.message.subject);
    /// if let Some(content) = &response.message.content {
    ///     println!("Content: {}", content);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn show(&self, id: u64) -> Result<MessageShowResponse, RavelryError> {
        let path = format!("messages/{}.json", id);
        let req = self.client.get(&path);
        self.client.send_json(req).await
    }

    /// Mark a message as read.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// client.messages().mark_read(12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_read(&self, id: u64) -> Result<MessageResponse, RavelryError> {
        let path = format!("messages/{}/mark_read.json", id);
        let req = self.client.post(&path);
        self.client.send_json(req).await
    }

    /// Mark a message as unread.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// client.messages().mark_unread(12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mark_unread(&self, id: u64) -> Result<MessageResponse, RavelryError> {
        let path = format!("messages/{}/mark_unread.json", id);
        let req = self.client.post(&path);
        self.client.send_json(req).await
    }

    /// Archive a message.
    ///
    /// Moves a message from inbox to the saved/archived folder.
    /// Requires the `message-write` OAuth scope.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// client.messages().archive(12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn archive(&self, id: u64) -> Result<MessageResponse, RavelryError> {
        let path = format!("messages/{}/archive.json", id);
        let req = self.client.post(&path);
        self.client.send_json(req).await
    }

    /// Delete a message.
    ///
    /// Requires the `message-write` OAuth scope.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// client.messages().delete(12345).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: u64) -> Result<MessageResponse, RavelryError> {
        let path = format!("messages/{}.json", id);
        let req = self.client.delete(&path);
        self.client.send_json(req).await
    }
}

/// Message folder types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageFolder {
    /// Inbox (received messages).
    Inbox,
    /// Sent messages.
    Sent,
    /// Archived/saved messages.
    Archived,
}

impl MessageFolder {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Inbox => "inbox",
            Self::Sent => "sent",
            Self::Archived => "archived",
        }
    }
}

impl Serialize for MessageFolder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Parameters for listing messages.
#[derive(Serialize, Default, Debug, Clone)]
pub struct MessagesListParams {
    /// Folder to list (required).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<MessageFolder>,

    /// Pagination parameters.
    #[serde(flatten)]
    pub page: PageParams,

    /// Only return unread messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_only: Option<bool>,

    /// Search term for fulltext searching messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// Output format ("list" or "full"). Defaults to "list".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,

    /// Sort order ("time" or "time_" for descending).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl MessagesListParams {
    /// Create new list params with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the folder to list.
    pub fn folder(mut self, folder: MessageFolder) -> Self {
        self.folder = Some(folder);
        self
    }

    /// Set the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page.page = Some(page);
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, size: u32) -> Self {
        self.page.page_size = Some(size);
        self
    }

    /// Only return unread messages.
    pub fn unread_only(mut self, unread: bool) -> Self {
        self.unread_only = Some(unread);
        self
    }

    /// Set a search term.
    pub fn search(mut self, query: impl Into<String>) -> Self {
        self.search = Some(query.into());
        self
    }

    /// Request full message content instead of list format.
    pub fn full_output(mut self) -> Self {
        self.output_format = Some("full".to_string());
        self
    }
}

/// Response from listing messages.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessagesListResponse {
    /// The list of messages.
    pub messages: Vec<MessageList>,

    /// Pagination information.
    pub paginator: Paginator,
}

/// Response from showing a single message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageShowResponse {
    /// The message details.
    pub message: MessageFull,
}

/// Response from message actions (mark read, archive, delete).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageResponse {
    /// The message.
    pub message: MessageFull,
}
