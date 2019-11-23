use chrono::{DateTime, Utc};

use super::super::{ChannelId, MessageId, UserId};

/// A direct message between multiple users.
pub struct GroupDMChannel {
    /// The id of this channel.
    pub id: ChannelId,
    /// Sorting position of the channel.
    pub position: u32,
    /// The name of the channel.
    ///
    /// (2-100 characters)
    pub name: String,
    /// Icon hash.
    pub icon: Option<String>,
    /// The id of the last message sent in this channel.
    ///
    /// May not point to an existing or valid message)
    pub last_message_id: Option<MessageId>,
    /// Id of the DM creator.
    pub owner_id: UserId,
    /// When the last pinned message was pinned.
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}