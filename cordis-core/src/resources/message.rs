use serde::Deserialize;

/// A snowflake if of a message.
#[derive(Deserialize)]
pub struct MessageId(String);