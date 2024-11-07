use crate::*;
use super::*;

// Define the event variants for token factory events
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
pub enum TokenFactoryEventKind {
    CreateDeployToken(CreateDeployTokenEvent),
}

// Define the main TokenFactoryEvent struct
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenFactoryEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: TokenFactoryEventKind,
}

impl TokenFactoryEvent {
    pub fn new(event: TokenFactoryEventKind) -> Self {
        TokenFactoryEvent {
            standard: "nep171-token-factory".to_string(),
            version: "1.0.0".to_string(),
            event,
        }
    }
}

impl std::fmt::Display for TokenFactoryEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

// Event for creating and deploying a token contract
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateDeployTokenEvent {
    pub token_id: AccountId,
    pub timestamp: u64,
}

impl CreateDeployTokenEvent {
    pub fn emit(self) {
        let event = TokenFactoryEvent::new(TokenFactoryEventKind::CreateDeployToken(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateDeployTokenEvent {
    fn event_kind(&self) -> &str {
        "create_deploy_token"
    }
}
