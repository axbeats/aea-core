use super::*;

// RemoveRoleFromAccountEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveRoleFromAccountEvent {
    pub account_id: AccountId,
    pub role_id: RoleId,
    pub timestamp: u64,
}

impl RemoveRoleFromAccountEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::RemoveRoleFromAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveRoleFromAccountEvent {
    fn event_kind(&self) -> &str {
        "remove_role_from_account"
    }
}