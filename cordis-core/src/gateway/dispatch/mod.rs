//! Contains all the structs for the Dispatch event.

use serde::{Deserialize, Serialize};

use super::Presence;
use super::super::resources::{Channel, Emoji, Guild, GuildId, GuildMember, Role, RoleId, User, UserId};

mod channel_pins_update;
pub use channel_pins_update::ChannelPinsUpdate;
mod guild_create;
pub use guild_create::{ClientStatus, GuildCreate, PartialPresenceUpdate, PartialVoiceState, PartialUser};
mod message;
pub use message::{MessageUpdate, MessageDelete, MessageDeleteBulk, MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll, UserWithMember};

/// An event dispatched from the server.
pub enum DispatchEvent {
    /// Sent when a new channel has been created, relative to the user.
    ChannelCreate(Channel),
    /// Sent when a channel is update.
    ///
    /// This is not sent when the `last_message_id` field is altered.
    ChannelUpdate(Channel),
    /// Sent when a channel is deleted.
    ChannelDelete(Channel),
    /// Sent when a message is pinned or unpinned.
    ///
    /// Not sent when a pinned message is deleted.
    ChannelPinsUpdate(ChannelPinsUpdate),
    /// Lazy load for unavailable guild, guild became available, or user joined a new guild.
    GuildCreate(GuildCreate),
    /// Sent when a guild is updated.
    GuildUpdate(Guild),
    /// Sent when a guild becomes unavailable during a guild outage, or when the user leaves or is removed form a guild.
    GuildDelete(UnavailableGuild),
    /// Sent when a user is banned from a guild.
    GuildBanAdd(GuildBan),
    /// Sent when a user is unbanned from a guild.
    GuildBanRemove(GuildBan),
    /// Sent when a guild's emojis have been updated.
    GuildEmojisUpdate(GuildEmojisUpdate),
    /// Sent when a guild integration is update.
    GuildIntegrationsUpdate(GuildIntegrationsUpdate),
    /// Sent when a user joins a guild.
    GuildMemberAdd(GuildMemberAdd),
    /// Sent when a user is removed from a guild (leaved/kicked/banned).
    GuildMemberRemove(GuildMemberRemove),
    /// Sent when a guild member is updated.
    GuildMemberUpdate(GuildMemberUpdate),
    /// Sent in response to a `GuildRequestMembers`.
    GuildMembersChunk(GuildMembersChunk),
    /// Sent when a guild role is added.
    GuildRoleAdd(GuildRole),
    /// Sent when a guild role is updated.
    GuildRoleUpdate(GuildRole),
    /// Sent when a guild role is deleted.
    GuildRoleDelete(GuildRoleDelete),
    /// Sent when a message has been created.
    MessageCreate(MessageUpdate),
    /// Sent when a message has been updated.
    MessageUpdate(MessageUpdate),
    /// Sent when a message is deleted.
    MessageDelete(MessageDelete),
    /// Sent when multiple messages are deleted at once.
    MessageDeleteBulk(MessageDeleteBulk),
    /// Sent when a user adds a reaction to a message.
    MessageReactionAdd(MessageReactionAdd),
    /// Sent when a user removes a reaction to a message.
    MessageReactionRemove(MessageReactionRemove),
    /// Sent when a user explicitly removes all reactions from a message.
    MessageReactionRemoveAll(MessageReactionRemoveAll),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DispatchEventCode {
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleAdd,
    GuildRoleUpdate,
    GuildRoleDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
}

/// A partial guild object.
#[derive(Deserialize)]
pub struct UnavailableGuild {
    /// The id of the guild.
    pub id: GuildId,
    /// If this is `false`, then the user was removed from the guild. 
    pub unavailable: bool,
}

/// Sent when a user is banned form a guild.
#[derive(Deserialize)]
pub struct GuildBan {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// The un/banned user.
    pub user: User,
}

/// Sent when a guild's emojis have been updated.
#[derive(Deserialize)]
pub struct GuildEmojisUpdate {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// The list of emojis on the guild.
    pub emojis: Vec<Emoji>,
}

/// Sent when a guild integration is updated.
#[derive(Deserialize)]
pub struct GuildIntegrationsUpdate {
    /// The guild that is being updated.
    pub guild_id: GuildId,
}

/// Sent when a user joins a guild.
#[derive(Deserialize)]
pub struct GuildMemberAdd {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// The member joining the guild.
    #[serde(flatten)]
    pub member: GuildMember,
}

/// Sent when a user is removed from a guild (leaved/kicked/banned).
#[derive(Deserialize)]
pub struct GuildMemberRemove {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// The user who was removed.
    pub user: User,
}

/// Sent when a guild member is updated.
#[derive(Deserialize)]
pub struct GuildMemberUpdate {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// The user's roles.
    pub roles: Vec<RoleId>,
    /// The user.
    pub user: User,
    /// The nickname of the user in the guild.
    pub nick: String,
}

/// Sent in response to a `GuildRequestMembers`.
#[derive(Deserialize)]
pub struct GuildMembersChunk {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// Set of guild members.
    pub members: Vec<GuildMember>,
    /// If passing invalid id to `RequestGuildMembers`, it will be returned here.
    pub not_found: Vec<UserId>,
    /// If passing `true` to `RequestGuildMembers`, presences will be returned here.
    pub presences: Vec<Presence>,
}

/// A role in a guild.
#[derive(Deserialize)]
pub struct GuildRole {
    /// Id of the guild.
    pub guild_id: GuildId,
    /// The Role created/updated.
    pub role: Role,
}

/// Sent when a guld role is deleted.
#[derive(Deserialize)]
pub struct GuildRoleDelete {
    /// Id of the guild.
    pub guild_id: GuildId,
    /// Id of the role that was deleted.
    pub role_id: RoleId,
}