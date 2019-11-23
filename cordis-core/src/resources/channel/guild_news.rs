use chrono::{DateTime, Utc};
use super::super::{ChannelId, GuildId, MessageId};

pub struct GuildNewsChannel {
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
    /// Id of the parent catergory for a channel.
    pub parent_id: Option<MessageId>,
    /// When the last pinned message was pinned.
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}