use super::*;

/// UpdateFeeEvent - Contract owner updates fee percentage
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateFeeEvent {
    pub old_fee_percentage: u32,
    pub new_fee_percentage: u32,
    pub updated_by: AccountId,
    pub timestamp: u64,
}

impl UpdateFeeEvent {
    pub fn emit(self) {
        let event = SubscriptionEvent::new(SubscriptionEventKind::UpdateFee(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateFeeEvent {
    fn event_kind(&self) -> &str {
        "update_fee"
    }
}
