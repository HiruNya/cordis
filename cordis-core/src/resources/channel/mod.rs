//! Contains the various Channel types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_repr::{Deserialize_repr};

use super::{GuildId, MessageId, UserId};

mod dm;
pub use dm::DMChannel;
mod guild_category;
pub use guild_category::GuildCategoryChannel;
mod group_dm;
pub use group_dm::GroupDMChannel;
mod guild_news;
pub use guild_news::GuildNewsChannel;
mod guild_store;
pub use guild_store::GuildStoreChannel;
mod guild_text;
pub use guild_text::GuildTextChannel;
mod guild_voice;
pub use guild_voice::GuildVoiceChannel;

/// A snowflake which is the id of a channel.
#[derive(Deserialize)]
pub struct ChannelId(String);

/// Represents a Discord guild or DM channel.
pub enum Channel {
    /// A text channel within a server.
    GuildText(GuildTextChannel),
    /// A direct message between users.
    DM(DMChannel),
    /// A voice channel within a server.
    GuildVoice(GuildVoiceChannel),
    /// A direct message between multiple users.
    GroupDm(GroupDMChannel),
    /// An organisational category that contains channels.
    GuildCategory(GuildCategoryChannel),
    /// A channel that users can follow and crosspost into their own server,
    GuildNews(GuildNewsChannel),
    /// A channel in which game developers can sell their game on Discord.
    GuildStore(GuildStoreChannel),
}

/// The type of channel it is.
#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum ChannelType {
    /// A text channel within a server.
    GuildText = 0,
    /// A direct message between users.
    Dm = 1,
    /// A voice channel within a server.
    GuildVoice = 2,
    /// A direct message between multiple users.
    GroupDm = 3,
    /// An organisational category that contains channels.
    GuildCategory = 4,
    /// A channel that users can follow and crosspost into their own server,
    GuildNews = 5,
    /// A channel in which game developers can sell their game on Discord.
    GuildStore = 6,
}

#[derive(Deserialize)]
struct InitialChannel {
    id: ChannelId,
    #[serde(rename = "type")]
    code: ChannelType,
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
            ChannelType::GuildText => Channel::GuildText(GuildTextChannel{
                id, last_message_id, parent_id, last_pin_timestamp,
                guild_id: guild_id.expect("Could not find `guild_id` for GuiltTextChannel."),
                position: position.expect("Could not find `position` for GuildTextChannel."),
                name: name.expect("Could not find `name` for GuildTextChannel."),
                topic: topic.expect("Could not find `topic` for GuildTextChannel."),
                nsfw: nsfw.expect("Could not find `nsfw` for GuildTextChannel."),
                rate_limit_per_user: rate_limit_per_user.expect("Could not find `rate_limit_per_user` for GuildTextChannel."),
            }),
            ChannelType::Dm => Channel::DM(DMChannel{
                id, last_message_id, last_pin_timestamp,
            }),
            ChannelType::GuildVoice => Channel::GuildVoice(GuildVoiceChannel{
                id, parent_id,
                guild_id: guild_id.expect("Could not find `guild_id` for GuildVoiceChannel."),
                position: position.expect("Could not find `position` for GUildVoiceChannel."),
                name: name.expect("Could not find `name` for GuildVoiceChannel."),
                nsfw: nsfw.expect("Could not find `nsfw` for GuildVoiceChannel."),
                bitrate: bitrate.expect("Could not find `bitrate` for GuildVoiceChannel."),
                user_limit: user_limit.expect("Could not find `user_limit` for GuildVoiceChannel."),
            }),
            ChannelType::GroupDm => Channel::GroupDm(GroupDMChannel{
                id, icon, last_message_id, last_pin_timestamp,
                name: name.expect("Could not find `name` for GroupD<Channel."),
                position: position.expect("Could not find `position` for GroupDMChannel."),
                owner_id: owner_id.expect("Could not find `owner_id` for GroupDMChannel."),
            }),
            ChannelType::GuildCategory => Channel::GuildCategory(GuildCategoryChannel{
                id, parent_id,
                name: name.expect("Could not find `name` for GuildCategoryChannel."),
                guild_id: guild_id.expect("Could not find `guild_id` for GuildCategoryChannel."),
                position: position.expect("Could not find `position` for GuildCategoryChannel."),
                nsfw: nsfw.expect("Could not find `nsfw` for GuildCategoryChannel."),
            }),
            ChannelType::GuildNews => Channel::GuildNews(GuildNewsChannel{
                id, parent_id, last_message_id, last_pin_timestamp,
                name: name.expect("Could not find `name` for GuildNewsChannel."),
                topic: topic.expect("Could not find `topic` for GuildNewsChannel."),
                guild_id: guild_id.expect("Could not find `guild_id` for GuildNewsChannel."),
                position: position.expect("Could not find `position` for GuildNewsChannel."),
                nsfw: nsfw.expect("Could not find `nsfw` for GuildNewsChannel."),
            }),
            ChannelType::GuildStore => Channel::GuildStore(GuildStoreChannel{
                id, parent_id,
                name: name.expect("Could not find `name` for GuildStoreChannel."),
                guild_id: guild_id.expect("Could not find `guild_id` for GuildStoreChannel."),
                nsfw: nsfw.expect("Could not find `nsfw` for GuildStoreChannel."),
                position: position.expect("Could not find `position` for GuildStoreChannel."),
            }),
        })
    }
}