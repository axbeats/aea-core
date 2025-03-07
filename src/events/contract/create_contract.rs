use super::*;
use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CreateContractEvent {
    pub contract: Contract,
    pub timestamp: u64,
}

impl CreateContractEvent {
    pub fn emit(self) {
        let event = ContractEvent::new(ContractEventKind::CreateContract(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateContractEvent {
    fn event_kind(&self) -> &str {
        "create_contract"
    }
}