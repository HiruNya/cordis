use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::super::super::resources::{GuildId, ChannelId};

/// Sent when a message is pinned or unpinned.
///
/// Not sent when a pinned message is deleted.
#[derive(Deserialize)]
pub struct ChannelPinsUpdate {
    /// The guild id which contains the channel.
    pub guild_id: Option<GuildId>,
    /// The if of the channel where the pins change.
    pub channel_id: ChannelId,
    /// The timestamp of the last pin.
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}