use super::{ChannelId, GuildId, MessageId};

/// An organisational category that contains channels.
pub struct GuildCategoryChannel {
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
}