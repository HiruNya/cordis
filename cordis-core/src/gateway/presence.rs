use serde::Deserialize;

use super::{Activity, Status};
use super::dispatch::{ClientStatus, PartialUser};
use super::super::resources::{GuildId, RoleId};

/// A user's presence is their current state on a guild.
#[derive(Deserialize)]
pub struct Presence {
    /// The id of the guild.
    pub guild_id: GuildId,
    /// The user whose presence is being updated.
    pub user: PartialUser,
    /// Roles the user is in.
    pub roles: Vec<RoleId>,
    /// The user's current activity.
    pub game: Option<Activity>,
    /// The status of the user.
    pub status: Status,
    /// User's current activities.
    pub activities: Vec<Activity>,
    /// User's platform-dependant status.
    pub client_status: ClientStatus,
}