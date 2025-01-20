use super::*;

// TransferNFTEvent
#[near(serializers = [json, borsh])]
#[derive(Debug)]
pub struct TransferNFTEvent {
    pub sender_id: AccountId,
    pub receiver_id: AccountId,
    pub token_id: TokenId,
    pub memo: Option<String>,
}

impl TransferNFTEvent {
    pub fn emit(self) {
        let event = NFTEvent::new(NFTEventKind::TransferNFT(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for TransferNFTEvent {
    fn event_kind(&self) -> &str {
        "transfer_nft"
    }
}
