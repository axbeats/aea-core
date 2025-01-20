use super::*;

// Event for unstaking tokens
#[near(serializers = [json])]
#[derive(Debug)]
pub struct UnstakeEvent {
    pub account_id: AccountId,
    pub amount: u128,
    pub timestamp: u64,
}

impl UnstakeEvent {
    pub fn emit(self) {
        let event = StakingEvent::new(StakingEventKind::Unstake(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnstakeEvent {
    fn event_kind(&self) -> &str {
        "unstake"
    }
}