use super::*;

// NftCreateVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CreateVideoEvent {
    pub video: Video,
}

impl CreateVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::CreateVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CreateVideoEvent {
    fn event_kind(&self) -> &str {
        "create_video"
    }
}