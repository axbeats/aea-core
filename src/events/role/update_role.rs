use super::*;

// UpdateRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateRoleEvent {
    pub role: Role,
    pub timestamp: u64,
}

impl UpdateRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::UpdateRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateRoleEvent {
    fn event_kind(&self) -> &str {
        "update_role"
    }
}