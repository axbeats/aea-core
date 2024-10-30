// Import all items from the current crate.
use crate::*;

// Define a type alias for CommentId, which is a u64.
pub type CommentId = u64;

/// Represents a comment made on a video.
///
/// Each comment is uniquely identified by its `id` and is associated with a specific video (`video_id`).
/// Comments can be made by users, and the `author` field captures the account ID of the user who made the comment.
/// Additionally, comments can have replies, and the `next_reply_id` is used to manage and generate unique identifiers for these replies.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
// Specify that Serde should use the `near_sdk::serde` crate for serialization.
#[serde(crate = "near_sdk::serde")]
pub struct Comment {
    pub id: CommentId,         // Unique identifier for the comment.
    pub video_id: VideoId,     // ID of the video this comment is associated with.
    pub author_id: AccountId,  // ID of the user who authored the comment.
    pub content: String,       // The content of the comment.
    pub issued_at: u64,        // Timestamp when the comment was issued, in Unix epoch milliseconds.
}

/// Represents the output data for a comment, including additional engagement metrics.
///
/// This struct provides a comprehensive view of a comment, including its core information, 
/// as well as additional data such as like count and reply count.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
// Specify that Serde should use the `near_sdk::serde` crate for serialization.
#[serde(crate = "near_sdk::serde")]
pub struct CommentOutput {
    pub id: CommentId,         // Unique identifier for the comment.
    pub video_id: VideoId,     // ID of the video this comment is associated with.
    pub author_id: AccountId,  // ID of the user who authored the comment.
    pub content: String,       // The content of the comment.
    pub issued_at: u64,        // Timestamp when the comment was issued, in Unix epoch milliseconds.
    pub does_account_like: bool, // Indicates if the current account likes this comment.
    pub like_count: u64,       // Number of likes the comment has received.
    pub reply_count: u64,      // Number of replies to the comment.
}
