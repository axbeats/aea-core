use super::*;

// MintNFTEvent
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintNFTEvent {
    pub token: VideoToken, // Assuming you have a VideoToken struct
}

impl MintNFTEvent {
    pub fn emit(self) {
        let event = NFTEvent::new(NFTEventKind::MintNFT(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for MintNFTEvent {
    fn event_kind(&self) -> &str {
        "mint_nft"
    }
}
