use serde::Deserialize;

/// Id snowflake of a User.
#[derive(Deserialize)]
pub struct UserId(pub String);