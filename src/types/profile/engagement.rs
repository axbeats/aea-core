use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProfileEngagement {
    pub follower_count: u64,
    pub following_count: u64,
    // pub subscriber_count: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ProfileEngagementKind {
    FollowerCount,
}