use serde::{Deserialize,Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Serialize)]
/// Sent by the client to indicate a presence or status update. 
pub struct StatusUpdate {
    /// Unix time (in milliseconds) of when the client went idle.
    ///
    /// `None` if the client is not idle.
    pub since: Option<u64>,
    /// The user's new activity.
    ///
    /// Or `None`.
    pub game: Option<Activity>,
    /// The user's new status.
    pub status: Status,
    /// Whether or not the client is AFK.
    pub afk: bool,
}

/// The user's activity.
#[derive(Deserialize, Serialize)]
pub struct Activity {
    /// The activity's name.
    pub name: String,
    /// Activity Type.
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    /// Stream URL.
    ///
    /// Is validated when type is 1.
    pub url: Option<String>,
}

/// The user's status.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Online.
    Online,
    /// Do Not Disturb.
    Dnd,
    /// AFK.
    Idle,
    /// Invisible and shown as Offline.
    Invisible,
    /// Offline.
    Offline,
}
impl Default for Status {
    fn default() -> Self {
        Status::Online
    }
}

/// The type of activity the user is doing.
#[derive(Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ActivityType {
    /// Playing {name}.
    ///
    /// e.g. "Playing Rocket League"
    Game = 0,
    /// Streaming {name}.
    ///
    /// e.g. "Streaming Rocker League"
    Streaming = 1,
    /// Listening to {name}.
    ///
    /// e.g. "Listening to Spotify"
    Listening = 2,
}
