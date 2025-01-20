use super::*;

// NftCommentVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct CommentVideoEvent {
    pub commenter_id: AccountId,
    pub video_id: VideoId,
    pub comment_id: CommentId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub content: String,
    pub timestamp: u64,
}

impl CommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::CommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for CommentVideoEvent {
    fn event_kind(&self) -> &str {
        "comment_video"
    }
}