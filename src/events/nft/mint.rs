use super::*;

// MintNFTEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct MintNFTEvent {
    pub token: VideoNFT, // Assuming you have a VideoToken struct
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
