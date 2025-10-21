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
pub struct VideoGraphPage {
    pub videos: Vec<VideoGraph>,
    pub count: i64,
    pub from_index: i64,
    pub limit: i64,
    pub has_more: bool,
}