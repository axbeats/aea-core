use super::*;

// Event for staking tokens
#[near(serializers = [json])]
#[derive(Debug)]
pub struct StakeEvent {
    pub account_id: AccountId,
    pub amount: u128,
    pub timestamp: u64,
}

impl StakeEvent {
    pub fn emit(self) {
        let event = StakingEvent::new(StakingEventKind::Stake(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for StakeEvent {
    fn event_kind(&self) -> &str {
        "stake"
    }
}