use super::*;

/// ActivateSubscriptionEvent - Creator reactivates their subscription
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct ActivateSubscriptionEvent {
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl ActivateSubscriptionEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::ActivateSubscription(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for ActivateSubscriptionEvent {
    fn event_kind(&self) -> &str {
        "activate_subscription"
    }
}
