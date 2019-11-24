use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{RoleId, User};

/// A member of a guild.
#[derive(Deserialize)]
pub struct GuildMember {
    /// The user this guild member represents.
    pub user: User,
    /// This user's guild nickname (if one is set).
    pub nick: Option<String>,
    /// Roles this user possesses in this guild.
    pub roles: Vec<RoleId>,
    /// When the user joined the guild.
    pub joined_at: DateTime<Utc>,
    /// When the user used their Nitro boost on this server.
    pub premium_since: Option<DateTime<Utc>>,
    /// Whether the user is deafened in voice channels.
    pub deaf: bool,
    /// Whether the user is muted in voice channels.
    pub mute: bool,
}