use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{ChannelId, super::{GuildId, MessageId}};

/// A text channel within a server.
#[derive(Deserialize)]
pub struct GuildTextChannel {
    /// The id of this channel.
    pub id: ChannelId,
    /// The id of the guild.
    pub guild_id: GuildId,
    /// Sorting position of the channel.
    pub position: u32,
    /// The name of the channel.
    ///
    /// (2-100 characters)
    pub name: String,
    /// The topic of the channel.
    ///
    /// (0-1024 characters)
    pub topic: String,
    /// Whether the channel is nsfw or not.
    pub nsfw: bool,
    /// The id of the last message sent in this channel.
    ///
    /// May not point to an existing or valid message)
    pub last_message_id: Option<MessageId>,
    /// Amount of seconds a user has to wait before sending another message.
    ///
    /// Value between 0 and 21600.
    ///
    /// Bots and Users with the permission ``manage_messages`` or ``manage_channel`` are unafected.
    pub rate_limit_per_user: u16,
    /// Id of the parent catergory for a channel.
    pub parent_id: MessageId,
    /// When the last pinned message was pinned.
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}
