use super::super::{ChannelId, GuildId, MessageId};

/// A voice channel within a server.
pub struct GuildVoiceChannel {
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
    /// Whether the channel is nsfw or not.
    pub nsfw: bool,
    /// Id of the parent catergory for a channel.
    pub parent_id: Option<MessageId>,
    /// The bitrate (in bits) of the voice channel.
    pub bitrate: u32,
    /// The user limit of the voice channel.
    pub user_limit: u32,
}