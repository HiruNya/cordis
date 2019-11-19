use serde::Serialize;

/// Sent when a client wants to join, move, or disconnect from a voice channel.
#[derive(Serialize)]
pub struct VoiceStateUpdate {
    /// Id of the guild.
    guid_id: String,
    /// Id of the channel client wants to join (none if disconnecting).
    channel_id: Option<String>,
    /// Is the client muted?
    self_mute: bool,
    /// Is the client deafened?
    self_deaf: bool,
}