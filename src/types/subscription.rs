use crate::*;

/// Subscription tier configuration for a creator
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct CreatorSubscription {
    pub creator_id: AccountId,
    pub ft_stream_contract: AccountId,
    pub rate_per_second: u128,
    pub active: bool,
}

/// Active subscription with stream tracking
#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ActiveSubscription {
    pub subscriber_id: AccountId,
    pub creator_id: AccountId,
    pub stream_id: FTStreamId,
    pub rate_per_second: u128,
    pub started_at: u64,
}