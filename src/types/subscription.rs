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
    pub inbound_stream_id: FTStreamId,  // subscriber → subscription contract
    pub rate_per_second: u128,           // total rate paid by subscriber
    pub fee_rate: u128,                  // contract keeps (revenue)
    pub forward_rate: u128,              // creator receives (rate - fee)
    pub started_at: u64,
}