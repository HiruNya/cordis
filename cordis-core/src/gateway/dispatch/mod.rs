//! Contains all the structs for the Dispatch event.

use serde::{Deserialize, Serialize};

use super::super::resources::Channel;

mod channel_pins_update;
pub use channel_pins_update::ChannelPinsUpdate;

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
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DispatchEventCode {
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
}