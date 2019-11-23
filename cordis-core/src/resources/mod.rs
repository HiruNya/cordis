//! Types that relate to common Discord elements like Messages and Channels.

pub mod channel;
pub use channel::{Channel, ChannelId};
mod guild;
pub use guild::GuildId;
mod message;
pub use message::MessageId;
mod user;
pub use user::UserId;
