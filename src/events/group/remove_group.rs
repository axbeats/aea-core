use super::*;

// RemoveGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveGroupEvent {
    pub group_id: GroupId,
    pub timestamp: u64,
}

impl RemoveGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::RemoveGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveGroupEvent {
    fn event_kind(&self) -> &str {
        "remove_group"
    }
}