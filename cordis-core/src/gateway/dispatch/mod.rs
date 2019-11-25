//! Contains all the structs for the Dispatch event.

use serde::{Deserialize, Serialize};

use super::super::resources::{Channel, Emoji, Guild, GuildId, User};

mod channel_pins_update;
pub use channel_pins_update::ChannelPinsUpdate;
mod guild_create;
pub use guild_create::{GuildCreate, PartialPresenceUpdate, PartialVoiceState, PartialUser};

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
    GuildEmojisUpdate(GuildEmojisUpdate)
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