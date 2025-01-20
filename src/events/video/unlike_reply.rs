use super::*;

// NftUnlikeReplyCommentVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct UnlikeReplyCommentVideoEvent {
    pub unliker_id: AccountId,
    pub reply_id: ReplyId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub timestamp: u64,
}

impl UnlikeReplyCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::UnlikeReplyCommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnlikeReplyCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "unlike_reply_comment_video"
    }
}