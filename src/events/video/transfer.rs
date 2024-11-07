use super::*;

// NftTransferVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TransferVideoEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<String>,
    pub old_owner_id: AccountId,
    pub new_owner_id: AccountId,
    pub video_id: VideoId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    pub timestamp: u64,
}

impl TransferVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::TransferVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for TransferVideoEvent {
    fn event_kind(&self) -> &str {
        "transfer_video"
    }
}