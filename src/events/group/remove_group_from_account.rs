use super::*;

// RemoveGroupFromAccountEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RemoveGroupFromAccountEvent {
    pub account_id: AccountId,
    pub group_id: GroupId,
    pub timestamp: u64,
}

impl RemoveGroupFromAccountEvent {
    pub fn emit(self) {
        let event = GroupEvent::new(GroupEventKind::RemoveGroupFromAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveGroupFromAccountEvent {
    fn event_kind(&self) -> &str {
        "remove_group_from_account"
    }
}