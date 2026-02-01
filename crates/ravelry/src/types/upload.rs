//! Upload-related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::common::ExtraFields;

/// A file to upload via the upload API.
#[derive(Debug, Clone)]
pub struct UploadFile {
    /// The filename to use in the multipart form.
    pub filename: String,
    /// The file contents as bytes.
    pub bytes: Vec<u8>,
    /// The content type (e.g., "image/jpeg"). If None, will be inferred.
    pub content_type: Option<String>,
}

impl UploadFile {
    /// Create a new upload file.
    pub fn new(filename: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self {
            filename: filename.into(),
            bytes,
            content_type: None,
        }
    }

    /// Set the content type.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

/// Response from requesting an upload token.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadRequestTokenResponse {
    /// The upload token to use for subsequent upload requests.
    pub upload_token: String,
}

/// Response from uploading images.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadImageResponse {
    /// Upload results keyed by file field name (file0, file1, etc.).
    pub uploads: Vec<HashMap<String, UploadResult>>,
}

/// Response from checking upload status.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadStatusResponse {
    /// Upload results keyed by file field name.
    pub uploads: Vec<HashMap<String, UploadResult>>,
}

/// Result for a single uploaded file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadResult {
    /// The image ID assigned to the upload.
    pub image_id: u64,

    /// Capture any additional fields.
    #[serde(flatten)]
    pub extra: ExtraFields,
}
