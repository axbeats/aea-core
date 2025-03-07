use super::*;
use crate::*;

pub use self::create_account::*;
pub use self::create_deploy_contract::*;
mod create_account;
mod create_deploy_contract;

// Define the event variants for profile events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum AccountFactoryEventKind {
    CreateAccount(CreateAccountEvent),
    CreateDeployContract(CreateDeployContractEvent),
}

// Define the main ProfileEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct AccountFactoryEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: AccountFactoryEventKind,
}

impl AccountFactoryEvent {
    pub fn new(event: AccountFactoryEventKind) -> Self {
        AccountFactoryEvent {
            standard: "nep171-account-factory".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for AccountFactoryEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}