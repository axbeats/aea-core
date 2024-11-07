use super::*;

// NftLikeCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LikeCommentVideoEvent {
    pub liker_id: AccountId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub commenter_id: AccountId,
    pub timestamp: u64,
}

impl LikeCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::LikeCommentVideo(self));
        env::log_str(&event.to_string());
    }
}


impl EventKind for LikeCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "like_comment_video"
    }
}