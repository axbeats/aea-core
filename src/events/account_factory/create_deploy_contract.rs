use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateDeployContractEvent {
    pub account_id: AccountId,
    pub bytes_hash: Vec<u8>,
    pub timestamp: u64,
}

impl CreateDeployContractEvent {
    pub fn emit(self) {
        let event = AccountFactoryEvent::new(AccountFactoryEventKind::CreateDeployContract(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateDeployContractEvent {
    fn event_kind(&self) -> &str {
        "create_deploy_contract"
    }
}