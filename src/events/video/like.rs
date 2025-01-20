use super::*;

// NftLikeVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct LikeVideoEvent {
    pub liker_id: AccountId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl LikeVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::LikeVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for LikeVideoEvent {
    fn event_kind(&self) -> &str {
        "like_video"
    }
}