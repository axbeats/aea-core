use super::*;

// RemoveAccountFromRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveAccountFromRoleEvent {
    pub role_id: RoleId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl RemoveAccountFromRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::RemoveAccountFromRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveAccountFromRoleEvent {
    fn event_kind(&self) -> &str {
        "remove_account_from_role"
    }
}