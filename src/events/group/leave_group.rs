use super::*;

// LeaveGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct LeaveGroupEvent {
    pub group_id: GroupId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl LeaveGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::LeaveGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for LeaveGroupEvent {
    fn event_kind(&self) -> &str {
        "leave_group"
    }
}