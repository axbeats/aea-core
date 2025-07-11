use super::*;

// LeaveRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct LeaveRoleEvent {
    pub role_id: RoleId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl LeaveRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::LeaveRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for LeaveRoleEvent {
    fn event_kind(&self) -> &str {
        "leave_role"
    }
}