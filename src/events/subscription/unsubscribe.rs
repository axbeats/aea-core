use super::*;

/// UnsubscribeEvent - Subscription cancelled
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UnsubscribeEvent {
    pub subscriber_id: AccountId,
    pub creator_id: AccountId,
    pub inbound_stream_id: FTStreamId,
    pub timestamp: u64,
}

impl UnsubscribeEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::Unsubscribe(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnsubscribeEvent {
    fn event_kind(&self) -> &str {
        "unsubscribe"
    }
}
