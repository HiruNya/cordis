//! The module to provide types that interface with the Websocket Gateway API.

use serde::{Deserialize, Serialize};

mod hello;
pub use hello::Hello;

/// The API version of the gateway this crate will support.
pub const VERSION: u8 = 6;

/// All the different operations that can be *received* by the client from the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
#[allow(missing_docs)]
#[derive(Deserialize)]
#[serde(tag = "op")]
pub enum ReceivedOperation {
    /// Dispatches an event.
    #[serde(rename = "0")]
    Dispatch,
    /// Used for ping checking.
    #[serde(rename = "1")]
    Heatbeat,
    /// Used to tell clients to reconnect to the gateway.
    #[serde(rename = "7")]
    Reconnect,
    /// Used to notify the client they have an invalid session id.
    #[serde(rename = "9")]
    InvalidSession,
    /// Sent immediately after connecting, contains heartbeat and server debug information.
    #[serde(rename = "10")]
    Hello{ data: Hello },
    /// Sent immediately following a client heartbeat that was received.
    #[serde(rename = "11")]
    HeartbeatACK, 
}

/// All the different operations that can be *sent* by the client to the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
#[derive(Serialize)]
#[serde(tag = "op")]
pub enum SendableOperation {
    /// Used for ping checking.
    #[serde(rename = "1")]
    Heatbeat,
    /// Used for client handshake.
    #[serde(rename = "2")]
    Identity,
    /// Used to update the client status.
    #[serde(rename = "3")]
    StatusUpdate,
    /// Used to join/move/leave voice channels.
    #[serde(rename = "4")]
    VoiceStateUpdate,
    /// Used to resume a closed connection.
    #[serde(rename = "6")]
    Resume,
    /// Used to request guild members.
    #[serde(rename = "8")]
    RequestGuildMembers,
}