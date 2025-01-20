use super::*;

// NftEditVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct EditVideoEvent {
    pub video: Video, // Not sure if I should emit entire video struct or an EditVideoEnum - Oct 30 2021
}

impl EditVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::EditVideo(self));
        env::log_str(&event.to_string());
    }
}


impl EventKind for EditVideoEvent {
    fn event_kind(&self) -> &str {
        "edit_video"
    }
}