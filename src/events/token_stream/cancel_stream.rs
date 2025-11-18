use super::*;

/// Event emitted when a payment stream is cancelled
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CancelStreamEvent {
    pub stream_id: u64,
    pub sender_id: AccountId,
    pub recipient_id: AccountId,
    pub cancelled_by: AccountId,
    pub timestamp: u64,
}

impl CancelStreamEvent {
    pub fn emit(self) {
        let event = TokenStreamEvent::new(TokenStreamEventKind::CancelStream(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CancelStreamEvent {
    fn event_kind(&self) -> &str {
        "cancel_stream"
    }
}
