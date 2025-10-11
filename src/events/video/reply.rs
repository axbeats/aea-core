use super::*;

// NftReplyCommentVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct ReplyCommentVideoEvent {
    pub replier_id: AccountId,
    pub comment_id: CommentId,
    pub reply_id: ReplyId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub content: String,
    pub timestamp: u64,
}

impl ReplyCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::ReplyCommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for ReplyCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "reply_comment_video"
    }
}