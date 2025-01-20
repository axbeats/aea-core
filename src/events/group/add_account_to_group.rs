use super::*;

// AddAccountToGroupEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct AddAccountToGroupEvent {
    pub group_id: GroupId,
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl AddAccountToGroupEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::AddAccountToGroup(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for AddAccountToGroupEvent {
    fn event_kind(&self) -> &str {
        "add_account_to_group"
    }
}