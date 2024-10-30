use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProfileEngagement {
    pub follower_count: u64,
    pub following_count: u64,
    // pub subscriber_count: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum ProfileEngagementKind {
    FollowerCount,
}