use serde::Deserialize;

use super::Emoji;

/// A reaction to a message.
#[derive(Deserialize)]
pub struct Reaction {
    /// Times this emoji has been used to react.
    pub count: u16,
    /// Whether the current user has reacted using this emoji.
    pub me: bool,
    /// The emoji used.
    pub emoji: Emoji,
}