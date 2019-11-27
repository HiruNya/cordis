use serde::Deserialize;

/// An id of an attachment.
#[derive(Deserialize)]
pub struct AttachmentId(pub String);

/// A file attached to a message.
#[derive(Deserialize)]
pub struct Attachment {
    /// Id of the attachment.
    pub id: AttachmentId,
    /// Name of the file.
    pub filename: String,
    /// Size of file in bytes.
    pub size: u64,
    /// Source URL of file.
    pub url: String,
    /// A proxied URL of file.
    pub proxy_url: String,
    /// If the attachment is an image, it will have the following dimensions.
    #[serde(flatten)]
    pub dimensions: Option<Dimensions>,
}

/// The dimensions of an image.
#[derive(Deserialize)]
pub struct Dimensions {
    /// Height of the file (if image).
    pub height: u16,
    /// Width of the file (if image).
    pub width: u16,
}