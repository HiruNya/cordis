use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_repr::{Deserialize_repr};

use super::{GuildId, MessageId, UserId};

mod dm;
pub use dm::DMChannel;
mod guild_text;
pub use guild_text::GuildTextChannel;

/// A snowflake which is the id of a channel.
#[derive(Deserialize)]
pub struct ChannelId(String);

/// Represents a Discord guild or DM channel.
pub enum Channel {
    /// A text channel within a server.
    GuildText(GuildTextChannel),
    /// A direct message between users.
    DM(DMChannel),
}

#[derive(Deserialize_repr)]
#[repr(u8)]
enum ChannelCode {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}

#[derive(Deserialize)]
struct InitialChannel {
    id: ChannelId,
    #[serde(rename = "type")]
    code: ChannelCode,
    guild_id: Option<GuildId>,
    position: Option<u32>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<MessageId>,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    rate_limit_per_user: Option<u16>,
    icon: Option<String>,
    owner_id: Option<UserId>,
    parent_id: Option<MessageId>,
    last_pin_timestamp: Option<DateTime<Utc>>,
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let InitialChannel{
            id, code,guild_id, position, name, topic,
            nsfw, last_message_id, rate_limit_per_user, parent_id, last_pin_timestamp,
            bitrate, user_limit, icon, owner_id,
        } = InitialChannel::deserialize(d)?;
        Ok(match code {
            ChannelCode::GuildText => Channel::GuildText(GuildTextChannel{
                id, last_message_id, parent_id, last_pin_timestamp,
                guild_id: guild_id.expect("Could not find `guild_id` for GuiltTextChannel."),
                position: position.expect("Could not find `position` for GuildTextChannel."),
                name: name.expect("Could not find `name` for GuildTextChannel."),
                topic: topic.expect("Could not find `topic` for GuildTextChannel."),
                nsfw: nsfw.expect("Could not find `nsfw` for GuildTextChannel."),
                rate_limit_per_user: rate_limit_per_user.expect("Could not find `rate_limit_per_user` for GuildTextChannel."),
            }),
            ChannelCode::Dm => Channel::DM(DMChannel{
                id, last_message_id, last_pin_timestamp,
            }),
            _ => Channel::DM(DMChannel{
                id, last_message_id, last_pin_timestamp,
            }),
        })
    }
}