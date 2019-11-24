use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// Id snowflake of a User.
#[derive(Deserialize)]
pub struct UserId(pub String);

/// A Discord User.
#[derive(Deserialize)]
pub struct User {
    /// The user's id.
    pub id: UserId,
    /// The user's username.
    ///
    /// This is not unique.
    pub username: String,
    /// The user's 4-digit discord tag.
    pub discriminator: String,
    /// The user's avatar hash.
    pub avatar: Option<String>,
    /// Whether the user belongs to an OAuth2 application.
    pub bot: Option<bool>,
    /// Whether the account has two factor authentication enabled on their account.
    pub mfa_enabled: Option<bool>,
    /// The user's chosen language option.
    pub locale: Option<String>,
    /// Whether the email on this account has been verified.
    pub verified: Option<bool>,
    /// The user's email.
    pub email: Option<String>,
    /// The flags on a user's account.
    pub flags: Option<u32>,
    /// The type of nitro subscription on a user's account.
    pub premium_type: Option<PremiumType>,
}

/// The type of premium account the user holds.
#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum PremiumType {
    /// Classic Nitro.
    NitroClassic = 1,
    /// Normal Nitro.
    Nitro = 2,
}