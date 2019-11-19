use serde::Serialize;

/// Used to request all members for a guild or list of guilds.
///
/// When initially connecting, the gateway will only send offline members if a guild has less than the
/// `large_threshold` members. If a client wishes to receive additional members, they need to explicitly
/// request them via this operation. The server will send Guild Member Chunk events in response
/// with up to 1000 members per chunk until all members that match the request have been sent.
///
/// Either the `query` or the `user_ids` field needs to be `Some`.
/// If you wish to get all members, just make the `query` `Some(String::new())`.
#[derive(Serialize)]
pub struct RequestGuildMembers {
    /// Id of the guild(s) to get members for.
    pub guild_id: Vec<String>,
    /// String that the username starts with, or an empty string to return all members.
    pub query: Option<String>, 
    /// maximum number of members to send matching the `query`; a limit of `0` can be used with an empty string
    /// to return all members.
    pub limit: u32,
    /// Used to specify if we want the presences of the matched members
    pub presences: bool,
    /// Used to specify which users you wish to fetch.
    pub user_ids : Option<Vec<String>>,
}
impl Default for RequestGuildMembers {
    fn default() -> Self {
        Self {
            guild_id: Vec::new(),
            query: Some(String::new()),
            limit: 0,
            presences: false,
            user_ids: None,
        }
    }
}