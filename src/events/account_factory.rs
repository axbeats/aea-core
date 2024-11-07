use crate::*;
use super::*;

// Define the event variants for profile events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum AccountFactoryEventKind {
    CreateAccount(CreateAccountEvent),
    CreateDeployContract(CreateDeployContractEvent),
}

// Define the main ProfileEvent struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

// CreateProfileEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateAccountEvent {
    pub account_id: AccountId,
    pub timestamp: u64,
}

impl CreateAccountEvent {
    pub fn emit(self) {
        let event = AccountFactoryEvent::new(AccountFactoryEventKind::CreateAccount(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateAccountEvent {
    fn event_kind(&self) -> &str {
        "create_account"
    }
}

// CreateProfileEvent
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