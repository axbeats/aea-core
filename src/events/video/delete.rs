use super::*;

// NftDeleteVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteVideoEvent {
    pub owner_id: AccountId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl EventKind for DeleteVideoEvent {
    fn event_kind(&self) -> &str {
        "delete_video"
    }
}

impl DeleteVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::DeleteVideo(self));
        env::log_str(&event.to_string());
    }
}