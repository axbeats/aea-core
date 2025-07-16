use super::*;
use crate::*;

pub use self::add_account_to_group::*;
pub use self::add_group_to_account::*;
pub use self::create_group::*;
pub use self::join_group::*;
pub use self::leave_group::*;
pub use self::remove_account_from_group::*;
pub use self::remove_group_from_account::*;
pub use self::remove_group::*;
pub use self::update_group::*;

mod add_account_to_group;
mod add_group_to_account;
mod create_group;
mod join_group;
mod leave_group;
mod remove_account_from_group;
mod remove_group_from_account;
mod remove_group;
mod update_group;

// Define the event variants for group events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum GroupEventKind {
    CreateGroup(CreateGroupEvent),
    RemoveGroup(RemoveGroupEvent),
    UpdateGroup(UpdateGroupEvent),
    AddAccountToGroup(AddAccountToGroupEvent),
    RemoveAccountFromGroup(RemoveAccountFromGroupEvent),
    AddGroupToAccount(AddGroupToAccountEvent),
    RemoveGroupFromAccount(RemoveGroupFromAccountEvent),
    JoinGroup(JoinGroupEvent),
    LeaveGroup(LeaveGroupEvent),
}

// Define the main GroupEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
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