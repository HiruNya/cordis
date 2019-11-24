use serde::Deserialize;
use serde_repr::Deserialize_repr;

use super::{Emoji, Role, ChannelId};

/// The id of a guild.
#[derive(Deserialize)]
pub struct GuildId(pub String);

/// A Discord Guild.
#[derive(Deserialize)]
pub struct Guild {
    /// The id of the guild.
    pub id: GuildId,
    /// The name of the guild.
    ///
    /// (2-1100 characters)
    pub name: String,
    /// Icon hash.
    pub icon: Option<String>,
    /// Splash hash.
    pub splash: Option<String>,
    /// Whether or not the user is the owner of the guild.
    pub owner: bool,
    /// Total permissions for the user in the guild (does not include channel overides).
    pub permissions: u32,
    /// Voice region id for the guild.
    pub region: String,
    /// Id of the AFK channel.
    pub afk_channel_id: Option<ChannelId>,
    /// AFK timeout in seconds.
    pub afk_timeout: u16,
    /// Whether this guild is embeddable (e.g. widget).
    pub embed_enabled: Option<bool>,
    /// The channel id that the widget will generate an invite to.
    pub embed_channel_id: Option<ChannelId>,
    /// Verification level required for the guild.
    pub verification_level: u32,
    /// The default message notification level.
    pub default_message_notifications: DefaultMessageNotificationLevel,
    /// The explicit content filter level.
    pub explicit_content_filter: ExplicitContentFilterLevel,
    /// The roles in the guild.
    pub roles: Vec<Role>,
    /// Custom guild emojis.
    pub emojis: Vec<Emoji>,
    /// Enabled guild features.
    pub features: Vec<GuildFeatures>,
    /// Required MFA level for this guild.
    pub mfa_level: MFALevel,
    /// Application id of the guild creator if it is bot-created.
    pub application_id: Option<String>,
    /// Whether or not the server widget is enabled.
    pub widget_enabled: Option<bool>,
    /// The channel id for the server widget.
    pub widget_channel_id: Option<ChannelId>,
    /// The id of the channel to which system messages are sent.
    pub system_channel_id: Option<ChannelId>,
    /// The maximum amount of presences for the guild.
    ///
    /// The default value is 5000 if `None`.
    pub max_presences: Option<u16>,
    /// The maximum amount of members for the guild.
    pub max_members: Option<u16>,
    /// The vanity url code of the guild.
    pub vanity_url_code: Option<String>,
    /// The description for the guild.
    pub description: Option<String>,
    /// The banner hash.
    pub banner: Option<String>,
    /// Premium tier.
    pub premium_tier: PremiumTier,
    /// The total number of users currently boosting this server.
    pub premium_subscription_count: Option<u16>,
    /// The preferred locale of this guild only set if the guild is `DISCOVERABLE`.
    ///
    /// Defaults to `en-US`.
    pub preferred_locale: String,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

/// An enabled guild feature.
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildFeatures {
    /// Guild has access to set an invite splash background.
    InviteSplash,
    /// Guild has access to set 384kbps bitrate in voice (previously VIP voice servers).
    VipRegions,
    /// Guild has access to set a vanity URL.
    VanityUrl,
    /// Guild is verified.
    Verified,
    /// Guild is partnered.
    Partnered,
    /// Guild is public.
    Public,
    /// Guild has access to use commerce features (i.e. create store channels).
    Commerce,
    /// Guild has access to create news channels.
    News,
    /// Guild is able to be discovered in the directory.
    Discoverable,
    /// Guild is able to be featured in the directory.
    Featurable,
    /// Guild has access to set an animated guild icon.
    AnimatedIcon,
    /// Guild has access to set a guild build banner image.
    Banner,
}
#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum MFALevel {
    None = 0,
    Elevated = 1,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}