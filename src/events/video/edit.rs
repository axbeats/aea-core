use super::*;

// NftEditVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct UpdateVideoEvent {
    pub video: Video, // Not sure if I should emit entire video struct or an UpdateVideoEnum - Oct 30 2021
}

impl UpdateVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::EditVideo(self));
        env::log_str(&event.to_string());
    }
}


impl EventKind for UpdateVideoEvent {
    fn event_kind(&self) -> &str {
        "update_video"
    }
}