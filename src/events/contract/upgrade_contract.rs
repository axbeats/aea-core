use super::*;

// I currently have this in the account-factory events because I didn't have an contract specific event folder - Feb 18 2025
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct UpgradeContractEvent {
    pub account_id: AccountId,
    pub bytes_hash: Vec<u8>,
    pub timestamp: u64,
}

impl UpgradeContractEvent {
    pub fn emit(self) {
        let event = ContractEvent::new(ContractEventKind::UpgradeContract(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpgradeContractEvent {
    fn event_kind(&self) -> &str {
        "upgrade_contract"
    }
}