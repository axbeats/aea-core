// Import all items from the current crate.
use crate::*;

// Import the AddAssign trait from the standard library to allow using the add_assign method.
use std::ops::AddAssign;

// Define a struct named `VideoEngagement` to hold various engagement metrics for a video.
// This struct derives several traits:
// - BorshDeserialize, BorshSerialize: for (de)serializing the struct using Borsh format.
// - Serialize, Deserialize: for (de)serializing the struct using Serde.
// - Clone: to allow the struct to be cloned.
// - Default: to allow the struct to be created with default values.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Default)]
pub struct VideoEngagement {
    pub view_count: u64,           // Number of views.
    pub view_length_in_ms: u64,    // Total view length in milliseconds.
    pub like_count: u64,           // Number of likes.
    pub favourite_count: u64,      // Number of times the video was favorited.
    pub comment_count: u64,        // Number of comments.
    pub share_count: u64,          // Number of shares.
    pub collaboration_count: u64,  // Number of collaborations.
}

// Implement the AddAssign trait for the `VideoEngagement` struct to allow adding another `VideoEngagement` instance to it.
impl AddAssign for VideoEngagement {
    // Define the add_assign method which takes a mutable reference to self and another `VideoEngagement` instance.
    fn add_assign(&mut self, other: Self) {
        // Add the counts from the other instance to the current instance.
        self.view_count += other.view_count;
        self.view_length_in_ms += other.view_length_in_ms;
        self.like_count += other.like_count;
        self.favourite_count += other.favourite_count;
        self.comment_count += other.comment_count;
        self.share_count += other.share_count;
        self.collaboration_count += other.collaboration_count;
    }
}

/// Enum representing the available sort methods for videos.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VideoEngagementKind {
    ViewCount,
    ViewLengthInMs,
    LikeCount,
    FavouriteCount,
    CommentCount,
    ShareCount,
    CollaborationCount,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Default)]
pub struct EngagementTimestamp {
    pub timestamp: u64,
    pub engagement: VideoEngagement,
}

/// Represents the engagement metrics comparison between a specific account and the entire platform for a given day.
///
/// This struct provides insights into how a particular account's engagement measures up 
/// against the overall platform engagement on a specific day.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Default)]
pub struct EngagementRatio {
    pub account_engagement: VideoEngagement,
    pub platform_engagement: VideoEngagement,
}