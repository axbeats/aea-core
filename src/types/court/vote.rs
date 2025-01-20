// use neo4rs::Node;

use crate::*;

pub type ReviewVoteId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ReviewVote {
    pub id: ReviewVoteId,
    pub dao_id: DaoId,
    pub rule_id: RuleId,
    pub review_id: ReviewId,
    pub account_id: AccountId,
    pub group_id: GroupId,
    pub vote: ReviewVoteKind,
    pub weight: u128,
    pub issued_at: TimestampNanoSeconds,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ReviewVoteInput {
    pub review_id: ReviewId,
    pub group_id: GroupId,
    pub vote: ReviewVoteKind,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, Default)]
pub struct ReviewVoteTally {
    pub innocent_count: u128,
    pub guilty_count: u128,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum ReviewVoteKind {
    Innocent,
    Guilty,
}