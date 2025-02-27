use crate::*;

pub type Weight = u128;

/// Todo: Delete this, replaced with VoteWeightKind
/// Will need to refactor process_vote()
/// Aug 6 2024
#[near(serializers = [json, borsh])]
#[derive(Clone, PartialEq, Debug, Hash)]
pub enum WeightKind {
    Account,        // 1 vote per account
    LinearToken,    // 1 vote per token
    QuadraticToken, // n votes per n^2 tokens
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub enum VoteWeightKind {
    Single,   // 1 vote per account
    Token((StakingId, WeightFormula)),    // votes per tokens    // I also have StakingId saved in GroupKind::Token (might be a way to amalgomate) - Aug 13 2024
    Follower(WeightFormula), // votes per followers
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct WeightInfo {
    pub account_weight: u128,
    pub total_weight: u128,
}