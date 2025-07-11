use super::*;

// CreateRoleEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct CreateRoleEvent {
    pub role: Role,
    pub timestamp: u64,
}

impl CreateRoleEvent {
    pub fn emit(self) {
        let event = RoleEvent::new(RoleEventKind::CreateRole(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateRoleEvent {
    fn event_kind(&self) -> &str {
        "create_role"
    }
}