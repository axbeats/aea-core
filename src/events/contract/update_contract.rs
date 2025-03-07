use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct UpdateContractEvent {
    pub contract: Contract,
    pub timestamp: u64,
}

impl UpdateContractEvent {
    pub fn emit(self) {
        let event = ContractEvent::new(ContractEventKind::UpdateContract(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UpdateContractEvent {
    fn event_kind(&self) -> &str {
        "update_contract"
    }
}