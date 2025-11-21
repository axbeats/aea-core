use super::*;

/// SetupSubscriptionEvent - Creator configures their subscription offering
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct SetupSubscriptionEvent {
    pub config: CreatorSubscription,
    pub timestamp: u64,
}

impl SetupSubscriptionEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::SetupSubscription(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for SetupSubscriptionEvent {
    fn event_kind(&self) -> &str {
        "setup_subscription"
    }
}
