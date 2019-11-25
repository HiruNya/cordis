use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_value, to_value, Value as JsonValue};

use super::{RecvOpCode, SendOpCode};
use super::{dispatch::{DispatchEvent, DispatchEventCode}, Hello, Identity, RequestGuildMembers, Resume, StatusUpdate, VoiceStateUpdate};

#[derive(Default, Deserialize, Serialize)]
struct InitialPayload<O> {
    op: O,
    d: Option<JsonValue>,
    s: Option<u32>,
    t: Option<DispatchEventCode>,
}

/// All the different payloads that can be *received* by the client from the server.
///
/// [See the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes)
pub enum ReceivedPayload {
    /// Dispatches an event.
    Dispatch {
        /// Sequence number used for heartbeats and resuming sessions.
        seq: u32,
        /// The event that has been dispatched by the server.
        event: DispatchEvent,
    },
    /// Used for ping checking.
    Heartbeat(Option<u32>),
    /// Used to tell clients to reconnect to the gateway.
    Reconnect,
    /// Indicates the current session is invalid.
    ///
    /// Caused by three things:
    /// * The gateway could not initialise a session after receiving an `Identity` command.
    /// * The gateway could not resume a previous session after receiving a `Resume` command.
    /// * The gateway has invalidated an active session and is requesting client action.
    ///
    /// The inner boolean indicates whether the session may be resumable.
    InvalidSession(bool),
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
            RecvOpCode::Heartbeat => Ok(ReceivedPayload::Heartbeat(from_value(initial_payload.d.expect("Expected data in Heartbeat event.")).expect("Could not parse `Heartbeat` Payload data"))),
            RecvOpCode::Reconnect => Ok(ReceivedPayload::Reconnect),
            RecvOpCode::InvalidSession => Ok(ReceivedPayload::InvalidSession(from_value(initial_payload.d.expect("Expected data in InvalidSession event")).expect("Could not parse `InvalidSession` Payload data"))),
            RecvOpCode::Hello => Ok(ReceivedPayload::Hello(from_value(initial_payload.d.expect("Expected data in Hello event.")).expect("Could not parse `Hello` Payload data"))),
            RecvOpCode::HeartbeatACK => Ok(ReceivedPayload::HeartbeatACK),
            RecvOpCode::Dispatch => match initial_payload.t.expect("Could not find Dispatch Event Code")  {
                DispatchEventCode::ChannelCreate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::ChannelCreate(from_value(initial_payload.d.expect("Expected data in `ChannelCreate` event.")).expect("Could not parse `ChannelCreate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `ChannelCreate` event"),
                }),
                DispatchEventCode::ChannelUpdate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::ChannelUpdate(from_value(initial_payload.d.expect("Expected data in `ChannelUpdate` event.")).expect("Could not parse `ChannelUpdate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `ChannelUpdate` event"),
                }),
                DispatchEventCode::ChannelDelete => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::ChannelDelete(from_value(initial_payload.d.expect("Expected data in `ChannelDelete` event.")).expect("Could not parse `ChannelDelete` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `ChannelDelete` event"),
                }),
                DispatchEventCode::ChannelPinsUpdate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::ChannelPinsUpdate(from_value(initial_payload.d.expect("Expected data in `ChannelPinsUpdate` event.")).expect("Could not parse `ChannelPinsUpdate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `ChanneldPinsUpdate` event"),
                }),
                DispatchEventCode::GuildCreate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildCreate(from_value(initial_payload.d.expect("Expected data in `GuildCreate` event.")).expect("Could not parse `GuildCreate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildCreate` event"),
                }),
                DispatchEventCode::GuildUpdate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildUpdate(from_value(initial_payload.d.expect("Expected data in `GuildUpdate` event.")).expect("Could not parse `GuildUpdate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildUpdate` event"),
                }),
                DispatchEventCode::GuildDelete => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildDelete(from_value(initial_payload.d.expect("Expected data in `GuildDelete` event.")).expect("Could not parse `GuildDelete` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildDelete` event"),
                }),
                DispatchEventCode::GuildBanAdd => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildBanAdd(from_value(initial_payload.d.expect("Expected data in `GuildBanAdd` event.")).expect("Could not parse `GuildBanAdd` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildBanAdd` event"),
                }),
                DispatchEventCode::GuildBanRemove => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildBanRemove(from_value(initial_payload.d.expect("Expected data in `GuildBanRemove` event.")).expect("Could not parse `GuildBanRemove` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildBanRemove` event"),
                }),
                DispatchEventCode::GuildEmojisUpdate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildEmojisUpdate(from_value(initial_payload.d.expect("Expected data in `GuildEmojisUpdate` event.")).expect("Could not parse `GuildEmojisUpdate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildEmojisUpdate` event"),
                }),
                DispatchEventCode::GuildIntegrationsUpdate => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildIntegrationsUpdate(from_value(initial_payload.d.expect("Expected data in `GuildIntegrationsUpdate` event.")).expect("Could not parse `GuildIntegrationsUpdate` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildIntegrationsUpdate` event"),
                }),
                DispatchEventCode::GuildMemberAdd => Ok(ReceivedPayload::Dispatch{
                    event: DispatchEvent::GuildMemberAdd(from_value(initial_payload.d.expect("Expected data in `GuildMemberAdd` event.")).expect("Could not parse `GuildMemberAdd` Payload data.")),
                    seq: initial_payload.s.expect("Expected sequence number in `GuildMemberAdd` event"),
                }),
            },
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
    StatusUpdate(StatusUpdate),
    /// Used to join/move/leave voice channels.
    VoiceStateUpdate(VoiceStateUpdate),
    /// Used to resume a closed connection.
    Resume(Resume),
    /// Used to request guild members.
    RequestGuildMembers(RequestGuildMembers),
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
                    d: Some(to_value(seq).expect("Error serialising `Option<u32>` for Heartbeat")),
                    ..InitialPayload::default()
                }
            },
            SendablePayload::Identity(identity) => {
                InitialPayload {
                    op: SendOpCode::Identity,
                    d: Some(to_value(&identity).expect("Error serialising `Identity` for Identity")),
                    ..InitialPayload::default()
                }
            },
            SendablePayload::StatusUpdate(status) => {
                InitialPayload {
                    op: SendOpCode::StatusUpdate,
                    d: Some(to_value(status).expect("Error serialising `StatusUpdate` for StatusUpdate")),
                    ..InitialPayload::default()
                }
            },
            SendablePayload::VoiceStateUpdate(status) => {
                InitialPayload {
                    op: SendOpCode::VoiceStateUpdate,
                    d: Some(to_value(status).expect("Error serialising `VoiceStatusUpdate` for VoiceStatusUpdate")),
                    ..InitialPayload::default()
                }
            },
            SendablePayload::Resume(resume) => {
                InitialPayload {
                    op: SendOpCode::Resume,
                    d: Some(to_value(resume).expect("Error serialising `Resume` for Resume")),
                    ..InitialPayload::default()
                }
            },
            SendablePayload::RequestGuildMembers(request) => {
                InitialPayload {
                    op: SendOpCode::RequestGuildMembers,
                    d: Some(to_value(request).expect("Error serialising `RequestGuildMembers` for RequestGuildMembers")),
                    ..InitialPayload::default()
                }
            },
        };
        InitialPayload::<SendOpCode>::serialize(&payload, serializer)
    }
}
