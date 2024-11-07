use super::*;

// NftCreateVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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