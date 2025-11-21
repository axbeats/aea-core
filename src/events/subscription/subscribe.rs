use super::*;

/// SubscribeEvent - Subscriber successfully subscribes to creator
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct SubscribeEvent {
    pub subscription: ActiveSubscription,
}

impl SubscribeEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::Subscribe(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for SubscribeEvent {
    fn event_kind(&self) -> &str {
        "subscribe"
    }
}
