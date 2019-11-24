use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::super::{Activity, Status};
use super::super::super::resources::{Channel, ChannelId, Guild, GuildMember, RoleId, PremiumType, UserId};

/// A guild that was either created or lazily-loaded.
#[derive(Deserialize)]
pub struct GuildCreate {
    /// The guild that was created.
    #[serde(flatten)]
    pub guild: Guild,
    /// When this guild was joined.
    pub joined_at: DateTime<Utc>,
    /// Whether this guild is considered to be a large guild.
    pub large: bool,
    /// Whether this guild is unavailable.
    pub unavailable: bool,
    /// Total number of members in this guild.
    pub member_count: u16,
    /// An array of voice states.
    pub voice_states: Vec<PartialVoiceState>,
    /// Members of the guild.
    pub members: Vec<GuildMember>,
    /// Channels in the guild.
    pub channels: Vec<Channel>,
    /// Presences of the users in the guild.
    pub presences: Vec<PartialPresenceUpdate>,
}

/// A voice state of a member of a guild.
#[derive(Deserialize)]
pub struct PartialVoiceState {
    /// The channel id this user is connected to.
    pub channel_id: Option<ChannelId>,
    /// The user id this voice state is for.
    pub user_id: UserId,
    /// The guild member this voice status is for.
    pub member: Option<GuildMember>,
    /// The session id for this voice state.
    pub session_id: String,
    /// Whether the user is deafened by the server.
    pub deaf: bool,
    /// Whether the user is muted by the server.
    pub mute: bool,
    /// Whether the user is locally deafened.
    pub self_deaf: bool,
    /// Whether the user is locally muted.
    pub self_mute: bool,
    /// Whether the user is streaming using "Go Live".
    pub self_stream: Option<bool>,
    /// Whether this user is muted by the current user.
    pub suppress: bool,
}

/// A user's presence is their current state on a guild.
#[derive(Deserialize)]
pub struct PartialPresenceUpdate {
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

/// Like the user struct but only the id is guaranteed. 
#[derive(Deserialize)]
pub struct PartialUser {
    /// The user's id.
    pub id: UserId,
    /// The user's username.
    ///
    /// This is not unique.
    pub username: Option<String>,
    /// The user's 4-digit discord tag.
    pub discriminator: Option<String>,
    /// The user's avatar hash.
    pub avatar: Option<String>,
    /// Whether the user belongs to an OAuth2 application.
    pub bot: Option<bool>,
    /// Whether the account has two factor authentication enabled on their account.
    pub mfa_enabled: Option<bool>,
    /// The user's chosen language option.
    pub locale: Option<String>,
    /// Whether the email on this account has been verified.
    pub verified: Option<bool>,
    /// The user's email.
    pub email: Option<String>,
    /// The flags on a user's account.
    pub flags: Option<u32>,
    /// The type of nitro subscription on a user's account.
    pub premium_type: Option<PremiumType>,
}

#[derive(Deserialize)]
pub struct ClientStatus {
    /// The user's status set for an active desktop (Windows, Linux, Mac) application session.
    pub desktop: Option<ClientSessionStatus>,
    /// The user's status set for an active mobile (iOS, Android) application session.
    pub mobile: Option<ClientSessionStatus>,
    /// The user's status set for an active web (browser, bot account) application session.
    pub web: Option<ClientSessionStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientSessionStatus {
    Online,
    Idle,
    Dnd,
}