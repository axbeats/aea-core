use super::*;

// AddGroupToAccountEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct AddGroupToAccountEvent {
    pub account_id: AccountId,
    pub group_id: GroupId,
    pub timestamp: u64,
}

impl AddGroupToAccountEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::AddGroupToAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for AddGroupToAccountEvent {
    fn event_kind(&self) -> &str {
        "add_group_to_account"
    }
}