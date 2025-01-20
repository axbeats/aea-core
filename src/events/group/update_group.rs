use super::*;

// UpdateGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpdateGroupEvent {
    pub group: Group,
    pub timestamp: u64,
}

impl UpdateGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::UpdateGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateGroupEvent {
    fn event_kind(&self) -> &str {
        "update_group"
    }
}