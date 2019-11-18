use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_value, to_value, Value as JsonValue};

use super::{RecvOpCode, SendOpCode};
use super::{Hello, Identity};

#[derive(Default, Deserialize, Serialize)]
struct InitialPayload<O> {
    op: O,
    d: JsonValue,
    s: Option<u32>,
    t: Option<String>,
}

/// All the different payloads that can be *received* by the client from the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
pub enum ReceivedPayload {
    /// Dispatches an event.
    Dispatch,
    /// Used for ping checking.
    Heartbeat,
    /// Used to tell clients to reconnect to the gateway.
    Reconnect,
    /// Used to notify the client they have an invalid session id.
    InvalidSession,
    /// Sent immediately after connecting, contains heartbeat and server debug information.
    Hello(Hello),
    /// Sent immediately following a client heartbeat that was received.
    /// 
    /// If the client doesn't receive this in-between sending `Heartbeat`s,
    /// close the connection with a non-1000 close code, reconnect, and attempt to resume.
    HeartbeatACK, 
}

impl<'de> Deserialize<'de> for ReceivedPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let initial_payload = InitialPayload::<RecvOpCode>::deserialize(deserializer)?;
        match initial_payload.op {
            RecvOpCode::Hello => Ok(ReceivedPayload::Hello(from_value(initial_payload.d).expect("Could not parse `Hello` Payload data"))),
            _ => Ok(ReceivedPayload::Heartbeat)
        }
    }
}

/// All the different payloads that can be *sent* by the client to the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
pub enum SendablePayload {
    /// Used for ping checking.
    ///
    /// Contains an ``Option<u32>`` which is the last sequence number, `s`, received by the client.
    Heartbeat(Option<u32>),
    /// Used for client handshake.
    Identity(Identity),
    /// Used to update the client status.
    StatusUpdate,
    /// Used to join/move/leave voice channels.
    VoiceStateUpdate,
    /// Used to resume a closed connection.
    Resume,
    /// Used to request guild members.
    RequestGuildMembers,
}

impl Serialize for SendablePayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let payload = match self {
            SendablePayload::Heartbeat(seq) => {
                InitialPayload {
                    op: SendOpCode::Heartbeat,
                    d: to_value(seq).expect("Error serialising `Option<u32>` for Heartbeat"),
                    ..InitialPayload::default()
                }
            },
            SendablePayload::Identity(identity) => {
                InitialPayload {
                    op: SendOpCode::Identity,
                    d: to_value(&identity).expect("Error serialising `Identity` for Identity"),
                    ..InitialPayload::default()
                }
            },
            _ => InitialPayload::default(), 
        };
        InitialPayload::<SendOpCode>::serialize(&payload, serializer)
    }
}
