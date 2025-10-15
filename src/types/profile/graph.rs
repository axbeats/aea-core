use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileGraph {
    pub profile: Profile,
    pub engagement: ProfileEngagement,
    pub video_engagement: VideoEngagement,
    pub interaction: ProfileInteraction,
    pub mutual_connections: Vec<ProfileSnapshot>,
    pub mutual_count: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileGraphs {
    pub profiles: Vec<ProfileGraph>,
    pub count: u64,
    pub from_index: u64,
    pub limit: u64,
    pub has_more: bool,
}