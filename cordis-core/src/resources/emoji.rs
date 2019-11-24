use serde::Deserialize;

use super::{Role, User};

/// The id of an emoji.
#[derive(Deserialize)]
pub struct EmojiId(pub String);

/// An emoji that was uploaded to a guild.
#[derive(Deserialize)]
pub struct Emoji {
    /// The id of the emoji.
    pub id: EmojiId,
    /// The name of the emoji.
    ///
    /// Can be `None` only in Reactions.
    pub name: Option<String>,
    /// Roles this emoji is whitelisted to.
    pub roles: Vec<Role>,
    /// User that created this role.
    pub user: Option<User>,
    /// Whether this emoji must be wrapped in colons.
    pub require_colons: Option<bool>,
    /// Whether this emoji is managed.
    pub managed: Option<bool>,
    /// Whether this emoji is animated.
    pub animated: Option<bool>,
}