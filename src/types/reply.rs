// Import all items from the current crate.
use crate::*;

// Define a type alias for ReplyId, which is a u64.
pub type ReplyId = u64;

/// Represents a reply to a comment made on a video.
///
/// Each reply is associated with a specific comment (`comment_id`) and video (`video_id`).
/// Replies can be made by users, and the `author` field captures the account ID of the user who made the reply.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Reply {
    pub id: ReplyId,           // Unique identifier for the reply.
    pub comment_id: CommentId, // ID of the comment this reply is associated with.
    pub video_id: VideoId,     // ID of the video this reply is associated with.
    pub author_id: AccountId,  // ID of the user who authored the reply.
    pub content: String,       // The content of the reply.
    pub issued_at: u64,        // Timestamp when the reply was issued, in Unix epoch milliseconds.
}

/// Represents the output data for a reply, including additional engagement metrics.
///
/// This struct provides a comprehensive view of a reply, including its core information,
/// as well as additional data such as like count.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ReplyOutput {
    pub id: ReplyId,           // Unique identifier for the reply.
    pub comment_id: CommentId, // ID of the comment this reply is associated with.
    pub video_id: VideoId,     // ID of the video this reply is associated with.
    pub author_id: AccountId,  // ID of the user who authored the reply.
    pub content: String,       // The content of the reply.
    pub issued_at: u64,        // Timestamp when the reply was issued, in Unix epoch milliseconds.
    pub does_account_like: bool, // Indicates if the current account likes this reply.
    pub like_count: u64,       // Number of likes the reply has received.
}
