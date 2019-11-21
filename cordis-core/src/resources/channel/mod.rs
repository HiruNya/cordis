use serde::Deserialize;

mod guild_text;
pub use guild_text::GuildTextChannel;

/// Represents a Discord guild or DM channel.
#[derive(Deserialize)]
pub enum Channel {
    /// A text channel within a server.
    GuildText(GuildTextChannel),
}

/// A snowflake which is the id of a channel.
#[derive(Deserialize)]
pub struct ChannelId(String);