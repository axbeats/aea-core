use crate::*;
use super::*;

pub use self::create_stream::*;
pub use self::cancel_stream::*;
pub use self::process_expiry::*;
pub use self::settle_account::*;

mod create_stream;
mod cancel_stream;
mod process_expiry;
mod settle_account;

#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum TokenStreamEventKind {
    CreateStream(CreateStreamEvent),
    CancelStream(CancelStreamEvent),
    ProcessExpiry(ProcessExpiryEvent),
    SettleAccount(SettleAccountEvent),
}

#[near(serializers = [json])]
#[derive(Debug)]
pub struct TokenStreamEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: TokenStreamEventKind,
}

impl TokenStreamEvent {
    pub fn new(event: TokenStreamEventKind) -> Self {
        TokenStreamEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for TokenStreamEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}
