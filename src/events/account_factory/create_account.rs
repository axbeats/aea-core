use super::*;

#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct CreateAccountEvent {
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl CreateAccountEvent {
    pub fn emit(self) {
        let event = AccountFactoryEvent::new(AccountFactoryEventKind::CreateAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateAccountEvent {
    fn event_kind(&self) -> &str {
        "create_account"
    }
}