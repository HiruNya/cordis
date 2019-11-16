//! The module to provide types that interface with the Websocket Gateway API.

/// The API version of the gateway this crate will support.
pub const VERSION: u8 = 6;

/// All the different operations that can be *received* by the client from the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
pub enum ReceivedOperation {
    /// Dispatches an event.
    Dispatch,
    /// Used for ping checking.
    Heatbeat,
    /// Used to tell clients to reconnect to the gateway.
    Reconnect,
    /// Used to notify the client they have an invalid session id.
    InvalidSession,
    /// Sent immediately after connecting, contains heartbeat and server debug information.
    Hello,
    /// Sent immediately following a client heartbeat that was received.
    HeartbeatACK, 
}

/// All the different operations that can be *sent* by the client to the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
pub enum SendableOperation {
    /// Used for ping checking.
    Heatbeat,
    /// Used for client handshake.
    Identity,
    /// Used to update the client status.
    StatusUpdate,
    /// Used to join/move/leave voice channels.
    VoiceStateUpdate,
    /// Used to resume a closed connection.
    Resume,
    /// Used to request guild members.
    RequestGuildMembers,
}