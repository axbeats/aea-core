use super::*;

// NftUnlikeVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UnlikeVideoEvent {
    pub unliker_id: AccountId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl UnlikeVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::UnlikeVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnlikeVideoEvent {
    fn event_kind(&self) -> &str {
        "unlike_video"
    }
}