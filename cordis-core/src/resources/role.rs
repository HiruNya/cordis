use serde::Deserialize;

/// The id of a role.
#[derive(Deserialize)]
pub struct RoleId(pub String);

/// A role that can be found in a guild.
#[derive(Deserialize)]
pub struct Role {
    /// The id of the role.
    pub id: RoleId,
    /// The name of the role.
    pub name: String,
    /// Integer representation of hexadecimal colour code.
    pub color: u32,
    /// If the role is pinned in the user listing.
    pub hoist: bool,
    /// Position of this role.
    pub position: u32,
    /// Permission bit set.
    pub permissions: u32,
    /// Whether this role is managed by an integration.
    pub managed: bool,
    /// Whether this role is mentionable.
    pub mentionable: bool,
}