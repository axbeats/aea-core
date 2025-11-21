use super::*;

/// PauseSubscriptionEvent - Creator pauses accepting new subscribers
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct PauseSubscriptionEvent {
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl PauseSubscriptionEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::PauseSubscription(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for PauseSubscriptionEvent {
    fn event_kind(&self) -> &str {
        "pause_subscription"
    }
}
