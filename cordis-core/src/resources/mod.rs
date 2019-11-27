//! Types that relate to common Discord elements like Messages and Channels.

mod attachment;
pub use attachment::{Attachment, Dimensions};
pub mod channel;
pub use channel::{Channel, ChannelId};
mod embed;
pub use embed::Embed;
mod emoji;
pub use emoji::{Emoji, EmojiId};
mod guild;
pub use guild::{Guild, GuildId};
mod guild_member;
pub use guild_member::GuildMember;
mod message;
pub use message::{Message, MessageId};
mod reaction;
pub use reaction::Reaction;
mod role;
pub use role::{Role, RoleId};
mod user;
pub use user::{PremiumType, User, UserId};
