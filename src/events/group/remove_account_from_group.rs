use super::*;

// RemoveAccountFromGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct RemoveAccountFromGroupEvent {
    pub group_id: GroupId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl RemoveAccountFromGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::RemoveAccountFromGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveAccountFromGroupEvent {
    fn event_kind(&self) -> &str {
        "remove_account_from_group"
    }
}