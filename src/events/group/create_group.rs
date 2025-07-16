use super::*;

// CreateGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct CreateGroupEvent {
    pub group: Group,
    pub timestamp: u64,
}

impl CreateGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::CreateGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateGroupEvent {
    fn event_kind(&self) -> &str {
        "create_group"
    }
}