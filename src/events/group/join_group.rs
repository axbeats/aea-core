use super::*;

// JoinGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct JoinGroupEvent {
    pub group_id: GroupId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl JoinGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::JoinGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for JoinGroupEvent {
    fn event_kind(&self) -> &str {
        "join_group"
    }
}