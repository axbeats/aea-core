use crate::*;

pub type FTStreamId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum FTStreamStatus {
    Active,
    Paused,
    Cancelled,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FTStream {
    pub from: AccountId,
    pub to: AccountId,
    pub rate: u128, // tokens per second
    pub start_time: u64,
    pub status: FTStreamStatus,
    pub service: Option<FTStreamService>, // None for P2P, Some for service subscriptions
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct FTStreamService {
    pub contract_id: AccountId,
    pub identifier: Option<String>,
}