use serde::Deserialize;

/// Returned when client connects to gateway.
#[derive(Deserialize)]
pub struct Hello {
    /// The interval (in milliseconds) between heartbeats that the client should be sending to the server.
    pub heartbeat_interval: u16,
}