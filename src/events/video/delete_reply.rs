use super::*;

// NftDeleteReplyCommentVideoLog
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeleteReplyCommentVideoEvent {
    pub replier_id: AccountId,
    pub comment_id: CommentId,
    pub reply_id: ReplyId,
    pub timestamp: u64,
}

impl DeleteReplyCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::DeleteReplyCommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteReplyCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "delete_reply_comment_video"
    }
}