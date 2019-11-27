use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::Dimensions;

/// An embed of a message.
#[derive(Deserialize)]
pub struct Embed {
    /// Title of the embed.
    pub embed: Option<String>,
    /// Type of embed.
    ///
    /// Always `rich` for webhook embeds.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Description of the embed.
    pub description: Option<String>,
    /// URL of the embed.
    pub url: Option<String>,
    /// Timestamp of embedded conten.
    pub timestamp: Option<DateTime<Utc>>,
    /// Colour code of the embed.
    pub color: u32,
    /// Footer information.
    pub footer: Option<EmbedFooter>,
    /// Image information.
    pub image: Option<EmbedImage>,
    /// Thumbnail information.
    pub thumbnail: Option<EmbedImage>,
    /// Video information.
    pub video: Option<EmbedVideo>,
    /// Provider information.
    pub provider: Option<EmbedProvider>,
    /// Author information.
    pub author: Option<EmbedAuthor>,
    /// Fields information.
    pub fields: Vec<EmbedField>,
}

/// The footer of an embed.
#[derive(Deserialize)]
pub struct EmbedFooter {
    /// Footer text.
    pub text: String,
    /// URL of footer icon (only supports http(s) and attachments).
    pub icon_url: Option<String>,
    /// A proxied URL of the footer icon.
    pub proxy_icon_url: Option<String>,
}

/// The image of an embed.
#[derive(Deserialize)]
pub struct EmbedImage {
    /// Source of the image (http(s) or attachment).
    pub url: Option<String>,
    /// Proxy url of the image.
    pub proxy_url: Option<String>,
    /// The dimensions of the image.
    #[serde(flatten)]
    pub dimensions: Option<Dimensions>,
}

/// The video of an embed.
#[derive(Deserialize)]
pub struct EmbedVideo {
    /// The source url of the video.
    pub url: Option<String>,
    /// The dimensions of the url.
    #[serde(flatten)]
    pub dimensions: Option<Dimensions>,
}

/// The provider of an embed.
#[derive(Deserialize)]
pub struct EmbedProvider {
    /// Name of provider.
    pub name: Option<String>,
    /// Url of provider.
    pub url: Option<String>,
}

/// The author of an embed.
#[derive(Deserialize)]
pub struct EmbedAuthor {
    /// The name of the author.
    pub name: Option<String>,
    /// The url of the author.
    pub url: Option<String>,
    /// The url source of the author icon.
    pub icon_url: Option<String>,
    /// The proxy url of the author icon.
    pub proxy_icon_url: Option<String>,
}

/// A field of an embed.
#[derive(Deserialize)]
pub struct EmbedField {
    /// Name of the field.
    pub name: String,
    /// Value of the field.
    pub value: String,
    /// Whether or not the field should be inline.
    pub inline: Option<bool>,
}