use super::*;

/// Event emitted when an account is settled (balance updated from streams)
#[near(serializers = [json])]
#[derive(Debug)]
pub struct SettleAccountEvent {
    pub account_id: AccountId,
    pub balance_before: u128,
    pub balance_after: u128,
    pub net_flow_rate: i128,
    pub elapsed_seconds: u64,
    pub timestamp: u64,
}

impl SettleAccountEvent {
    pub fn emit(self) {
        let event = TokenStreamEvent::new(TokenStreamEventKind::SettleAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for SettleAccountEvent {
    fn event_kind(&self) -> &str {
        "settle_account"
    }
}
