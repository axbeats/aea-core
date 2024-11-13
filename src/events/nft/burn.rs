use super::*;

// BurnNFTEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BurnNFTEvent {
    pub token_id: TokenId,             // Assuming TokenId is defined in your code
    pub timestamp: TimestampNanoSeconds, // Assuming TimestampNanoSeconds is defined in your code
}

impl BurnNFTEvent {
    pub fn emit(self) {
        let event = NFTEvent::new(NFTEventKind::BurnNFT(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for BurnNFTEvent {
    fn event_kind(&self) -> &str {
        "burn_nft"
    }
}
