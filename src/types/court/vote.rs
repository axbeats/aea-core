// use neo4rs::Node;

use crate::*;

pub type ReviewVoteId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ReviewVote {
    pub id: ReviewVoteId,
    pub dao_id: DaoId,
    pub rule_id: RuleId,
    pub review_id: ReviewId,
    pub account_id: AccountId,
    pub group_id: GroupId,
    pub vote: ReviewVoteKind,
    pub weight: u128,
    pub issued_at: TimestampMilliSeconds,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ReviewVoteInput {
    pub review_id: ReviewId,
    pub group_id: GroupId,
    pub vote: ReviewVoteKind,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct ReviewVoteTally {
    pub innocent_count: u128,
    pub guilty_count: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ReviewVoteKind {
    Innocent,
    Guilty,
}