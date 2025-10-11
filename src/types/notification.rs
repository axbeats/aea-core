use crate::*;

pub type NotificationId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: NotificationId,
    pub recipient_id: AccountId,
    pub sender_id: AccountId,
    pub kind: NotificationKind,
    pub issued_at: TimestampNanoSeconds,
    pub read: bool,
    pub read_at: Option<TimestampNanoSeconds>,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum NotificationKind {
    Follow,
    Like(VideoId),
    Comment(VideoId, CommentId),
    Reply(VideoId, ReplyId),
    LikeComment(VideoId, CommentId),
    LikeReply(VideoId, ReplyId),
    Favourite(VideoId),
    Share(VideoId),
    Transfer((AccountId, u128)),
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct NotificationResults {
    pub notifications: Vec<Notification>,
    pub count: u64,
    pub from_index: u64,
    pub limit: u64,
    pub has_more: bool,
}