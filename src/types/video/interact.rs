use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum InteractVideo {
    Like(VideoId),
    Unlike(VideoId),
    Comment(VideoId, String),
    RemoveComment(CommentId),
    Favourite(VideoId),
    Unfavourite(VideoId),
    Share(VideoId),
    Unshare(VideoId),
    Collaborate(VideoId),
}