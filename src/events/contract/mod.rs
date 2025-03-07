use super::*;
use crate::*;

pub use self::create_contract::*;
pub use self::update_contract::*;
pub use self::upgrade_contract::*;
pub use self::remove_contract::*;

mod create_contract;
mod update_contract;
mod upgrade_contract;
mod remove_contract;

// Define the event variants for contract events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum ContractEventKind {
    CreateContract(CreateContractEvent),
    UpdateContract(UpdateContractEvent),
    UpgradeContract(UpgradeContractEvent),
    RemoveContract(RemoveContractEvent),
}

#[near(serializers = [json])]
#[derive(Debug)]
pub struct ContractEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: ContractEventKind,
}

impl ContractEvent {
    pub fn new(event: ContractEventKind) -> Self {
        ContractEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for ContractEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}