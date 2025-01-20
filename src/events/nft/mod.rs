use super::*;
use crate::*;

pub use self::mint::*;
pub use self::burn::*;
pub use self::transfer::*;

mod mint;
mod burn;
mod transfer;

// Define the event variants for NFT events
#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum NFTEventKind {
    MintNFT(MintNFTEvent),
    BurnNFT(BurnNFTEvent),
    TransferNFT(TransferNFTEvent),
}

// Define the main NFTEvent struct
#[near(serializers = [json])]
#[derive(Debug)]
pub struct NFTEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: NFTEventKind,
}

impl NFTEvent {
    pub fn new(event: NFTEventKind) -> Self {
        NFTEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for NFTEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}
