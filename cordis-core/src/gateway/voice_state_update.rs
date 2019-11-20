use serde::Serialize;

/// Sent when a client wants to join, move, or disconnect from a voice channel.
#[derive(Serialize)]
pub struct VoiceStateUpdate {
    /// Id of the guild.
    pub guid_id: String,
    /// Id of the channel client wants to join (none if disconnecting).
    pub channel_id: Option<String>,
    /// Is the client muted?
    pub self_mute: bool,
    /// Is the client deafened?
    pub self_deaf: bool,
}
