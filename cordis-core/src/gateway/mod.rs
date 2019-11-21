//! The module to provide types that interface with the Websocket Gateway API.

use serde_repr::{Deserialize_repr, Serialize_repr};

mod payload;
pub use payload::{ReceivedPayload, SendablePayload};
pub mod dispatch;
mod hello;
pub use hello::Hello;
mod identity;
pub use identity::{ConnectionProperties, Identity};
mod status_update;
pub use status_update::{Activity, ActivityType, Status, StatusUpdate};
mod request_guild_members;
pub use request_guild_members::RequestGuildMembers;
mod resume;
pub use resume::Resume;
mod voice_state_update;
pub use voice_state_update::VoiceStateUpdate;

/// The API version of the gateway this crate will support.
pub const VERSION: u8 = 6;

/// All the different operations that can be *received* by the client from the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
#[allow(missing_docs)]
#[derive(Deserialize_repr)]
#[repr(u8)]
enum RecvOpCode {
    /// Dispatches an event.
    Dispatch = 0,
    /// Used for ping checking.
    Heartbeat = 1,
    /// Used to tell clients to reconnect to the gateway.
    Reconnect = 7,
    /// Used to notify the client they have an invalid session id.
    InvalidSession = 9,
    /// Sent immediately after connecting, contains heartbeat and server debug information.
    Hello = 10,
    /// Sent immediately following a client heartbeat that was received.
    /// 
    /// If the client doesn't receive this in-between sending `Heartbeat`s,
    /// close the connection with a non-1000 close code, reconnect, and attempt to resume.
    HeartbeatACK = 11, 
}

/// All the different operations that can be *sent* by the client to the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
#[derive(Serialize_repr)]
#[repr(u8)]
enum SendOpCode {
    /// Used for ping checking.
    Heartbeat = 1,
    /// Used for client handshake.
    Identity = 2,
    /// Used to update the client status.
    StatusUpdate = 3,
    /// Used to join/move/leave voice channels.
    VoiceStateUpdate = 4,
    /// Used to resume a closed connection.
    Resume = 6,
    /// Used to request guild members.
    RequestGuildMembers = 8,
}

impl Default for RecvOpCode {
    fn default() -> Self {
        Self::Heartbeat
    }
}

impl Default for SendOpCode {
    fn default() -> Self {
        Self::Heartbeat
    }
}
