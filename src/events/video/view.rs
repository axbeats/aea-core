use super::*;

// NftViewVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct ViewVideoEvent {
    pub viewer_id: AccountId,
    pub video_id: VideoId,
    pub view_count: u64,
    pub view_length_in_ms: u64,
    pub timestamp: u64,
}

impl ViewVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::ViewVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for ViewVideoEvent {
    fn event_kind(&self) -> &str {
        "view_video"
    }
}