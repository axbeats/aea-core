use super::*;

/// UpdateSubscriptionRateEvent - Creator changes their subscription price
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateSubscriptionRateEvent {
    pub creator_id: AccountId,
    pub old_rate: u128,
    pub new_rate: u128,
    pub timestamp: u64,
}

impl UpdateSubscriptionRateEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::UpdateSubscriptionRate(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateSubscriptionRateEvent {
    fn event_kind(&self) -> &str {
        "update_subscription_rate"
    }
}
