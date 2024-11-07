use super::*;

// NftFavouriteVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FavouriteVideoEvent {
    pub favouriter_id: AccountId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl FavouriteVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::FavouriteVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for FavouriteVideoEvent {
    fn event_kind(&self) -> &str {
        "favourite_video"
    }
}