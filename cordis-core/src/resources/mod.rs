//! Types that relate to common Discord elements like Messages and Channels.

mod channel;
pub use channel::{Channel, ChannelId};
mod guild;
pub use guild::GuildId;
mod message;
pub use message::MessageId;
