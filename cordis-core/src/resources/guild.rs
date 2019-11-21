use serde::Deserialize;

/// The id of a guild.
#[derive(Deserialize)]
pub struct GuildId(String);