use super::*;
use crate::*;

pub use self::create_deploy_token::*;
pub use self::create_deploy_ft_sale::*;

mod create_deploy_token;
mod create_deploy_ft_sale;

// Define the event variants for token factory events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum TokenFactoryEventKind {
    CreateDeployToken(CreateDeployTokenEvent),
    CreateFTSale(CreateFTSaleEvent),
}

// Define the main TokenFactoryEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
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