use crate::*;
use super::*;

// Define the event variants for group events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum GroupEventKind {
    CreateGroup(CreateGroupEvent),
    RemoveGroup(RemoveGroupEvent),
    UpdateGroup(UpdateGroupEvent),
    AddAccountToGroup(AddAccountToGroupEvent),
    RemoveAccountFromGroup(RemoveAccountFromGroupEvent),
    AddGroupToAccount(AddGroupToAccountEvent),
    RemoveGroupFromAccount(RemoveGroupFromAccountEvent),
}

// Define the main GroupEvent struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: GroupEventKind,
}

impl GroupEvent {
    pub fn new(event: GroupEventKind) -> Self {
        GroupEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for GroupEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// CreateGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// RemoveGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// UpdateGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// AddAccountToGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AddAccountToGroupEvent {
    pub group_id: String,
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

// RemoveAccountFromGroupEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// AddGroupToAccountEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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
