use super::*;

// RemoveRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveRoleEvent {
    pub role_id: RoleId,
    pub timestamp: u64,
}

impl RemoveRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::RemoveRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveRoleEvent {
    fn event_kind(&self) -> &str {
        "remove_role"
    }
}