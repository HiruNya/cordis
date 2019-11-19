use serde::Serialize;

#[derive(Serialize)]
/// Used to replay missed events when a disconnected client resumes.
pub struct Resume {
    /// Session token.
    pub token: String,
    /// Session id.
    pub session_id: String,
    /// Last sequence number received.
    pub seq: u32,
}