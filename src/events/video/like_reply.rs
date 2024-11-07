use super::*;

// NftLikeReplyCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LikeReplyCommentVideoEvent {
    pub liker_id: AccountId,
    pub reply_id: ReplyId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl LikeReplyCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::LikeReplyCommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for LikeReplyCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "like_reply_comment_video"
    }
}