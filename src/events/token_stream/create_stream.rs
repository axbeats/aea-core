use super::*;

/// Event emitted when a new payment stream is created
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CreateStreamEvent {
    pub stream_id: u64,
    pub sender_id: AccountId,
    pub recipient_id: AccountId,
    pub rate_per_second: u128,
    pub start_time: u64,
    pub timestamp: u64,
}

impl CreateStreamEvent {
    pub fn emit(self) {
        let event = TokenStreamEvent::new(TokenStreamEventKind::CreateStream(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateStreamEvent {
    fn event_kind(&self) -> &str {
        "create_stream"
    }
}
