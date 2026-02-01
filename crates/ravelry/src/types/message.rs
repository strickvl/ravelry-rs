//! Message types for the Ravelry API.
//!
//! Private messages between Ravelry users.

use serde::{Deserialize, Serialize};

use super::common::ExtraFields;
use super::user::UserSmall;

/// Message information returned in lists.
///
/// This is a minimal representation suitable for displaying in message lists.
/// Use [`MessageFull`] for complete message details.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageList {
    /// Unique message ID.
    pub id: u64,

    /// Message subject line.
    pub subject: String,

    /// Whether the message has been read.
    #[serde(default)]
    pub read_message: Option<bool>,

    /// The sender's information.
    #[serde(default)]
    pub sender: Option<UserSmall>,

    /// When the message was sent.
    #[serde(default)]
    pub sent_at: Option<String>,

    /// The folder this message is in.
    #[serde(default)]
    pub folder_name: Option<String>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Full message information including content.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageFull {
    /// Unique message ID.
    pub id: u64,

    /// Message subject line.
    pub subject: String,

    /// Whether the message has been read.
    #[serde(default)]
    pub read_message: Option<bool>,

    /// The sender's information.
    #[serde(default)]
    pub sender: Option<UserSmall>,

    /// The recipient's information.
    #[serde(default)]
    pub recipient: Option<UserSmall>,

    /// When the message was sent.
    #[serde(default)]
    pub sent_at: Option<String>,

    /// The folder this message is in.
    #[serde(default)]
    pub folder_name: Option<String>,

    /// Plain text message content.
    #[serde(default)]
    pub content: Option<String>,

    /// HTML-formatted message content.
    #[serde(default)]
    pub content_html: Option<String>,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Message data for sending a new message.
///
/// Requires the `message-write` OAuth scope.
#[derive(Serialize, Debug, Default, Clone)]
pub struct MessagePost {
    /// Message subject line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Message content (plain text).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Recipient user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_user_id: Option<u64>,

    /// Recipient username (alternative to user ID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_username: Option<String>,

    /// Capture any additional fields for flexibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl MessagePost {
    /// Create a new empty message.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the subject.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set the content.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Set the recipient by user ID.
    pub fn recipient_user_id(mut self, id: u64) -> Self {
        self.recipient_user_id = Some(id);
        self
    }

    /// Set the recipient by username.
    pub fn recipient_username(mut self, username: impl Into<String>) -> Self {
        self.recipient_username = Some(username.into());
        self
    }
}
