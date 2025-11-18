use super::*;

/// Event emitted when expired accounts are processed
#[near(serializers = [json])]
#[derive(Debug)]
pub struct ProcessExpiryEvent {
    pub account_id: AccountId,
    pub streams_cancelled: u32,
    pub balance_before: u128,
    pub balance_after: u128,
    pub timestamp: u64,
}

impl ProcessExpiryEvent {
    pub fn emit(self) {
        let event = TokenStreamEvent::new(TokenStreamEventKind::ProcessExpiry(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for ProcessExpiryEvent {
    fn event_kind(&self) -> &str {
        "process_expiry"
    }
}
