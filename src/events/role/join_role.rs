use super::*;

// JoinRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct JoinRoleEvent {
    pub role_id: RoleId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl JoinRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::JoinRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for JoinRoleEvent {
    fn event_kind(&self) -> &str {
        "join_role"
    }
}