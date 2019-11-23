use chrono::{DateTime, Utc};

use super::super::{ChannelId, MessageId};

/// A direct message between users.
pub struct DMChannel {
    /// The id of this channel.
    pub id: ChannelId,
    /// The id of the last message sent in this channel.
    ///
    /// May not point to an existing or valid message)
    pub last_message_id: Option<MessageId>,
    /// When the last pinned message was pinned.
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}
