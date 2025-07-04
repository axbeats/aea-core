// Import all items from the current crate.
use crate::*;

/// Represents the output data for a video, including its core information, engagement metrics, and interactions.
///
/// This struct aggregates various aspects of a video to provide a comprehensive view of its state and performance.
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoOutput {
    pub video: Video,                   // The core information about the video.
    pub engagement: VideoEngagement,    // The engagement metrics associated with the video.
    pub interaction: VideoInteraction,  // The interaction data related to the video.
}
