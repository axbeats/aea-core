use super::*;

// NftDeleteCommentVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct DeleteCommentVideoEvent {
    pub commenter_id: AccountId,
    pub video_id: VideoId,
    pub comment_id: CommentId,
    pub timestamp: u64,
}

impl DeleteCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::DeleteCommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for DeleteCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "delete_comment_video"
    }
}