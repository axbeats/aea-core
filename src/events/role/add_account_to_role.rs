use super::*;

// AddAccountToRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct AddAccountToRoleEvent {
    pub role_id: RoleId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl AddAccountToRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::AddAccountToRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for AddAccountToRoleEvent {
    fn event_kind(&self) -> &str {
        "add_account_to_role"
    }
}