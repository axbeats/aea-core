use super::*;
use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct RemoveContractEvent {
    pub contract_id: ContractId,
    pub dao_id: DaoId,
    pub timestamp: u64,
}

impl RemoveContractEvent {
    pub fn emit(self) {
        let event = ContractEvent::new(ContractEventKind::RemoveContract(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for RemoveContractEvent {
    fn event_kind(&self) -> &str {
        "remove_contract"
    }
}