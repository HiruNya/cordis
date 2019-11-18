use serde::{Serialize, Serializer};

use super::StatusUpdate;

/// Information used for the client handshake.
#[derive(Serialize)]
pub struct Identity {
    /// Authentication Token.
    pub token: String,
    /// Connection properties.
    pub properties: ConnectionProperties,
    /// Whether this connection supports compression of packets.
    /// This crate is not able to yet.
    ///
    /// `false` by default.
    pub compress: bool,
    /// Value between 50 and 250, total number of members where the gateway will stop sending offline members in the guild member list.
    ///
    /// `50` by default.
    pub large_threshold: u16,
    /// Used for [Guild Sharding](https://discordapp.com/developers/docs/topics/gateway#sharding)
    pub shard: Shard,
    /// Initial presence information.
    pub presence: StatusUpdate,
    /// Enables dispatching of guild subscription events (presence and typing events).
    ///
    /// `true` by default.
    pub guild_subscription: bool,
}

impl Default for Identity {
    fn default() -> Self {
        Identity {
            token: String::default(),
            properties: ConnectionProperties::default(),
            compress: false,
            large_threshold: 50,
            shard: Shard::default(),
            presence: StatusUpdate::default(),
            guild_subscription: true,
        }
    }
}

/// The properties of the connection the server, given when identifying.
///
/// See [the official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/gateway#identify-identify-connection-properties)
#[derive(Serialize)]
pub struct ConnectionProperties {
    /// The Operating System. e.g. Linux.
    #[serde(rename = "$os")]
    pub os: String,
    /// The Browser used, in this case "Cordis" is used by default.
    #[serde(rename = "$browser")]
    pub browser: String,
    /// The Device use, in this case "Cordis" is used by default.
    #[serde(rename = "$device")]
    pub device: String,
}
impl Default for ConnectionProperties {
    fn default() -> Self {
        Self {
            os: String::new(),
            browser: String::from("Cordis"),
            device: String::from("Cordis"),
        }
    }
}

/// Used for guild sharding.
///
/// [See Official Discord documentation for more information.](https://discordapp.com/developers/docs/topics/gateway#sharding)
pub struct Shard {
    /// The id of the current shard.
    pub shard_id: u16,
    /// The number of shards.
    pub num_shards: u16,
}
impl Default for Shard {
    fn default() -> Self {
        Self {
            shard_id: 0,
            num_shards: 1,
        }
    }
}
impl Serialize for Shard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.shard_id)?;
        seq.serialize_element(&self.num_shards)?;
        seq.end()
    }
}