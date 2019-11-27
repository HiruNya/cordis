use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use super::{Attachment, ChannelId, channel::ChannelType, Embed, Emoji, GuildId, RoleId, User};

/// A snowflake if of a message.
#[derive(Deserialize)]
pub struct MessageId(String);

/// Represents a Discord message.
#[derive(Deserialize)]
pub struct Message {
    /// Id of the message.
    pub id: MessageId,
    /// Id of the channel the message was sent in.
    pub channel_id: ChannelId,
    /// Id of the guild the message was sent in.
    pub guild_id: Option<GuildId>,
    /// The author of this message.
    ///
    /// If there is a webhook id, then the id is not of an actual author
    /// but instead the id of a webhook.
    pub author: User,
    /// The content of the message.
    pub content: String,
    /// When this message was sent.
    pub timestamp: DateTime<Utc>,
    /// When this message was edited.
    ///
    /// `None` if it was never edited.
    pub edited_timestamp: Option<DateTime<Utc>>,
    /// Whether this was a TTS message.
    pub tts: bool,
    /// Whether this message mentions everyone.
    pub mention_everyone: bool,
    /// Roles specifically mentioned in this message.
    pub mention_roles: Vec<RoleId>,
    /// Channels specifically mentioned in this message.
    ///
    /// Not all channel mentions in a message will appear in mention_channels.
    /// Only textual channels that are visible to everyone in a lurkable guild will ever be included.
    /// Only crossposted messages (via Channel Following) currently include mention_channels at all.
    pub mention_channels: Vec<ChannelMention>,
    /// Any attached files.
    pub attachments: Vec<Attachment>,
    /// Any embedded content.
    pub embeds: Vec<Embed>,
    /// Reactions to the message.
    pub reactions: Vec<Emoji>,
    /// Used for validating a message was sent.
    pub nonce: Option<Nonce>,
    /// Whether this message is pinned.
    pub pinned: bool,
    /// The type of message.
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// Sent with Rich-Presence related chat embeds.
    pub activity: Option<MessageActivity>,
    /// Sent with Rich Presence-related chat embeds
    pub application: Option<MessageApplication>,
    /// Reference data sent with crossposted messages.
    pub message_reference: Option<MessageReference>,
}

/// Used for validating a message was sent.
///
/// Is either an integer or a [`String`], hence the separate struct.
#[derive(Deserialize)]
pub enum Nonce {
    Text(String),
    Integer(u32),
}

/// The type of a message.
#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecepientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
}

/// Sent with Rich-Presence related chat embeds.
#[derive(Deserialize)]
pub struct MessageActivity {
    /// Type of message activity.
    #[serde(rename = "type")]
    pub type_: MessageActivityType,
    /// Party id from a rich presence event.
    pub party_id: String,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

/// An application that sent the message.
#[derive(Deserialize)]
pub struct MessageApplication {
    /// Id of the application.
    pub id: String,
    /// Id of the embed's image asset.
    pub cover_image: Option<String>,
    /// Application's description.
    pub description: String,
    /// id of the applicatio's icon.
    pub icon: Option<String>,
    /// Name of the application.
    pub name: String,
}

/// Reference data sent with crossposted messages.
#[derive(Deserialize)]
pub struct MessageReference {
    /// Id of the originating message.
    pub message_id: Option<MessageId>,
    /// Id of the originating message's channel.
    pub channel_id: ChannelId,
    /// Id of the originating message's guild.
    pub guild_id: Option<GuildId>,
}

/// Channels mentioned in a message.
#[derive(Deserialize)]
pub struct ChannelMention {
    /// Id of the channel.
    pub id: ChannelId,
    /// Id of the guild the channel is in.
    pub guild_id: GuildId,
    /// The type of the channel.
    #[serde(rename = "type")]
    pub type_: ChannelType,
    /// The name of the channel.
    pub name: String,
}