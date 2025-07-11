use super::*;
use crate::*;

pub use self::add_account_to_role::*;
pub use self::add_role_to_account::*;
pub use self::create_role::*;
pub use self::join_role::*;
pub use self::leave_role::*;
pub use self::remove_account_from_role::*;
pub use self::remove_role_from_account::*;
pub use self::remove_role::*;
pub use self::update_role::*;

mod add_account_to_role;
mod add_role_to_account;
mod create_role;
mod join_role;
mod leave_role;
mod remove_account_from_role;
mod remove_role_from_account;
mod remove_role;
mod update_role;

// Define the event variants for role events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum RoleEventKind {
    CreateRole(CreateRoleEvent),
    RemoveRole(RemoveRoleEvent),
    UpdateRole(UpdateRoleEvent),
    AddAccountToRole(AddAccountToRoleEvent),
    RemoveAccountFromRole(RemoveAccountFromRoleEvent),
    AddRoleToAccount(AddRoleToAccountEvent),
    RemoveRoleFromAccount(RemoveRoleFromAccountEvent),
    JoinRole(JoinRoleEvent),
    LeaveRole(LeaveRoleEvent),
}

// Define the main RoleEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct RoleEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: RoleEventKind,
}

impl RoleEvent {
    pub fn new(event: RoleEventKind) -> Self {
        RoleEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for RoleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}