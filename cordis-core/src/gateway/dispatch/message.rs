use serde::Deserialize;

use super::super::super::resources::{ChannelId, Emoji, GuildMember, GuildId, Message, MessageId, User, UserId};

/// Sent when a message has been created.
#[derive(Deserialize)]
pub struct MessageUpdate {
    /// The message itself.
    #[serde(flatten)]
    pub message: Message,
    /// The guild member that sent this message.
    pub member: GuildMember,
    /// Users specifically mentioned in the message.
    pub mentions: Vec<UserWithMember>,
}

/// A user with an additional member field.
#[derive(Deserialize)]
pub struct UserWithMember {
    /// The additional guild member field.
    pub member: GuildMember,
    /// The user field.
    #[serde(flatten)]
    pub user: User,
}

/// Sent when a message is deleted.
#[derive(Deserialize)]
pub struct MessageDelete {
    /// Id of the message.
    pub id: MessageId,
    /// Id of the channel the message was in.
    pub channel_id: ChannelId,
    /// Id of the guild the message was in.
    pub guild_id: Option<GuildId>,
}

/// Sent when multiple messages are deleted at once.
#[derive(Deserialize)]
pub struct MessageDeleteBulk {
    /// Ids of the messages.
    pub ids: Vec<MessageId>,
    /// Id of the channel the message was in.
    pub channel_id: ChannelId,
    /// Id of the guild the message was in.
    pub guild_id: Option<GuildId>,
}

/// Sent when a user adds a reaction to a message.
#[derive(Deserialize)]
pub struct MessageReactionAdd {
    /// Id of the user.
    pub user_id: UserId,
    /// Id of the channel.
    pub channel_id: ChannelId,
    /// Id of the message.
    pub message_id: MessageId,
    /// Id of the guild.
    pub guild_id: Option<GuildId>,
    /// The guild member that reacted.
    pub member: Option<GuildMember>,
    /// The emoji used in the reaction.
    pub emoji: Emoji,
}

/// Sent when a user removes a reaction to a message.
#[derive(Deserialize)]
pub struct MessageReactionRemove {
    /// Id of the user.
    pub user_id: UserId,
    /// Id of the channel.
    pub channel_id: ChannelId,
    /// Id of the message.
    pub message_id: MessageId,
    /// Id of the guild.
    pub guild_id: Option<GuildId>,
    /// The emoji used in the reaction.
    pub emoji: Emoji,
}

/// Sent when a user explicitly removes all reactions from a message.
#[derive(Deserialize)]
pub struct MessageReactionRemoveAll {
    /// Id of the channel.
    pub channel_id: ChannelId,
    /// Id of the message.
    pub message_id: MessageId,
    /// Id of the guild.
    pub guild_id: Option<GuildId>,
}