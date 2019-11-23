//! Contains all the structs for the Dispatch event.

use serde::{Deserialize, Serialize};

use super::super::resources::Channel;

/// An event dispatched from the server.
pub enum DispatchEvent {
    /// Sent when a new channel has been created, relative to the user.
    ChannelCreate(Channel),
    /// Sent when a channel is update.
    ///
    /// This is not sent when the `last_message_id` field is altered.
    ChannelUpdate(Channel),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DispatchEventCode {
    ChannelCreate,
    ChannelUpdate,
}