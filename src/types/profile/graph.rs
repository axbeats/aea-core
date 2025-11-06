use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileGraph {
    pub profile: Profile,
    pub profile_video: Option<VideoMedia>,
    pub engagement: ProfileEngagement,
    pub video_engagement: VideoEngagement,
    pub video_activity: VideoEngagement,
    pub interaction: ProfileInteraction,
    pub mutual_connections: Vec<ProfileSnapshot>,
    pub mutual_count: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileGraphPage {
    pub profiles: Vec<ProfileGraph>,
    pub count: i64,
    pub from_index: i64,
    pub limit: i64,
    pub has_more: bool,
}