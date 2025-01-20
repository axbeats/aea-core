use super::*;

// NftUnlikeCommentVideoLog
#[near(serializers = [json])]
#[derive(Debug)]
pub struct UnlikeCommentVideoEvent {
    pub unliker_id: AccountId,
    pub comment_id: CommentId,
    pub video_id: VideoId,
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub timestamp: u64,
}

impl UnlikeCommentVideoEvent {
    pub fn emit(self) {
        let event = VideoEvent::new(VideoEventKind::UnlikeCommentVideo(self));
        env::log_str(&event.to_string());
    }
}

impl EventKind for UnlikeCommentVideoEvent {
    fn event_kind(&self) -> &str {
        "unlike_comment_video"
    }
}