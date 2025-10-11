use super::*;

// NftUnfavouriteVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct UnfavouriteVideoEvent {
    pub unfavouriter_id: AccountId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub timestamp: u64,
}

impl UnfavouriteVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::UnfavouriteVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnfavouriteVideoEvent {
    fn event_kind(&self) -> &str {
        "unfavourite_video"
    }
}