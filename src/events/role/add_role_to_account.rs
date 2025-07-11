use super::*;

// AddRoleToAccountEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct AddRoleToAccountEvent {
    pub account_id: AccountId,
    pub role_id: RoleId,
    pub timestamp: u64,
}

impl AddRoleToAccountEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::AddRoleToAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for AddRoleToAccountEvent {
    fn event_kind(&self) -> &str {
        "add_role_to_account"
    }
}