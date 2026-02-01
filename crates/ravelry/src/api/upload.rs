//! Upload-related API endpoints.
//!
//! The upload flow is:
//! 1. Request an upload token via `request_token()`
//! 2. Upload images via `image()` (multipart, unauthenticated)
//! 3. Check status via `image_status()`

use reqwest::multipart::{Form, Part};

use crate::client::RavelryClient;
use crate::error::RavelryError;
use crate::types::{UploadFile, UploadImageResponse, UploadRequestTokenResponse, UploadStatusResponse};

/// Maximum number of files per upload request.
pub const MAX_UPLOAD_FILES: usize = 10;

/// Service for upload-related API endpoints.
pub struct UploadApi<'a> {
    pub(crate) client: &'a RavelryClient,
}

impl<'a> UploadApi<'a> {
    /// Request an upload token.
    ///
    /// This token is required for subsequent image uploads.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// let response = client.upload().request_token().await?;
    /// println!("Upload token: {}", response.upload_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request_token(&self) -> Result<UploadRequestTokenResponse, RavelryError> {
        let req = self.client.post("upload/request_token.json");
        self.client.send_json(req).await
    }

    /// Upload one or more images.
    ///
    /// **Note:** This endpoint is unauthenticated. The upload token provides authorization.
    ///
    /// # Arguments
    ///
    /// * `upload_token` - Token from `request_token()`
    /// * `files` - Up to 10 files to upload
    ///
    /// # Errors
    ///
    /// Returns an error if more than 10 files are provided.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// use ravelry::types::UploadFile;
    ///
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// // First get a token
    /// let token_resp = client.upload().request_token().await?;
    ///
    /// // Read file bytes
    /// let bytes = std::fs::read("photo.jpg")?;
    /// let file = UploadFile::new("photo.jpg", bytes)
    ///     .content_type("image/jpeg");
    ///
    /// // Upload
    /// let response = client.upload().image(&token_resp.upload_token, vec![file]).await?;
    /// for upload in response.uploads {
    ///     for (key, result) in upload {
    ///         println!("{}: image_id = {}", key, result.image_id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn image(
        &self,
        upload_token: &str,
        files: impl IntoIterator<Item = UploadFile>,
    ) -> Result<UploadImageResponse, RavelryError> {
        let files: Vec<UploadFile> = files.into_iter().collect();

        if files.len() > MAX_UPLOAD_FILES {
            return Err(RavelryError::InvalidRequest(format!(
                "Maximum {} files per upload, got {}",
                MAX_UPLOAD_FILES,
                files.len()
            )));
        }

        if files.is_empty() {
            return Err(RavelryError::InvalidRequest(
                "At least one file is required".to_string(),
            ));
        }

        // Build multipart form
        let mut form = Form::new().text("upload_token", upload_token.to_string());

        for (i, file) in files.into_iter().enumerate() {
            let field_name = format!("file{}", i);
            let mut part = Part::bytes(file.bytes).file_name(file.filename);

            if let Some(content_type) = file.content_type {
                part = part.mime_str(&content_type).map_err(|e| {
                    RavelryError::InvalidRequest(format!("Invalid content type: {}", e))
                })?;
            }

            form = form.part(field_name, part);
        }

        // Use unauthenticated POST for upload
        let req = self.client.post_no_auth("upload/image.json").multipart(form);
        self.client.send_json(req).await
    }

    /// Check the status of an upload.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ravelry::{RavelryClient, auth::BasicAuth};
    /// # async fn example() -> Result<(), ravelry::RavelryError> {
    /// # let client = RavelryClient::builder(BasicAuth::new("", "")).build()?;
    /// # let upload_token = "token";
    /// let status = client.upload().image_status(upload_token).await?;
    /// for upload in status.uploads {
    ///     for (key, result) in upload {
    ///         println!("{}: image_id = {}", key, result.image_id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn image_status(
        &self,
        upload_token: &str,
    ) -> Result<UploadStatusResponse, RavelryError> {
        let req = self
            .client
            .get_no_auth("upload/image/status.json")
            .query(&[("upload_token", upload_token)]);
        self.client.send_json(req).await
    }
}
