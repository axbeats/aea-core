use crate::*;
use super::*;

pub use self::comment::*;
pub use self::create::*;
pub use self::delete::*;
pub use self::delete_comment::*;
pub use self::delete_reply::*;
pub use self::edit::*;
pub use self::favourite::*;
pub use self::like::*;
pub use self::like_comment::*;
pub use self::like_reply::*;
pub use self::reply::*;
pub use self::unfavourite::*;
pub use self::unlike::*;
pub use self::unlike_comment::*;
pub use self::unlike_reply::*;
pub use self::view::*;

mod comment;
pub mod create;
mod delete;
mod delete_comment;
mod delete_reply;
mod edit;
mod favourite;
mod like;
mod like_comment;
mod like_reply;
mod reply;
mod unfavourite;
mod unlike;
mod unlike_comment;
mod unlike_reply;
mod view;

#[near(serializers = [json])]
#[derive(Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum VideoEventKind {
    CreateVideo(CreateVideoEvent),
    EditVideo(EditVideoEvent),
    DeleteVideo(DeleteVideoEvent),
    ViewVideo(ViewVideoEvent),
    LikeVideo(LikeVideoEvent),
    UnlikeVideo(UnlikeVideoEvent),
    FavouriteVideo(FavouriteVideoEvent),
    UnfavouriteVideo(UnfavouriteVideoEvent),
    CommentVideo(CommentVideoEvent),
    DeleteCommentVideo(DeleteCommentVideoEvent),
    ReplyCommentVideo(ReplyCommentVideoEvent),
    DeleteReplyCommentVideo(DeleteReplyCommentVideoEvent),
    LikeCommentVideo(LikeCommentVideoEvent),
    UnlikeCommentVideo(UnlikeCommentVideoEvent),
    LikeReplyCommentVideo(LikeReplyCommentVideoEvent),
    UnlikeReplyCommentVideo(UnlikeReplyCommentVideoEvent),
}

#[near(serializers = [json])]
#[derive(Debug)]
pub struct VideoEvent {
    pub standard: String,
    pub version: String,
    #[serde(flatten)]
    pub event: VideoEventKind,
}

impl VideoEvent {
    pub fn new(event: VideoEventKind) -> Self {
        VideoEvent {
            standard: EVENT_STANDARD_NAME.to_string(),
            version: EVENT_VERSION.to_string(),
            event,
        }
    }
}

impl std::fmt::Display for VideoEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT_JSON:{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}
