use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoGraph {
    pub video: Video,
    pub context: VideoContextGraph,
    pub engagement: VideoEngagement,
    pub interaction: VideoInteraction,
    pub creator_profile: ProfileSnapshot,
    pub mutual_connections: Vec<ProfileSnapshot>,
    pub mutual_count: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct VideoGraphs {
    pub videos: Vec<VideoGraph>,
    pub count: u64,
    pub from_index: u64,
    pub limit: u64,
    pub has_more: bool,
}