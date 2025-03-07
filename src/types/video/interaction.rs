use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoInteraction {
    pub viewed: bool,
    pub liked: bool,
    pub favourited: bool,
    pub commented: bool,
    pub shared: bool,
    pub collaborated: bool,
}

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