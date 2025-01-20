use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ReviewOutput {
    pub review: Review,
    pub vote_tally: ReviewVoteTally,
    // pub engagement: VoteEngagement,
    pub user_vote: Option<ReviewVote>,
}