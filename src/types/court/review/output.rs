use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ReviewOutput {
    pub review: Review,
    pub vote_tally: ReviewVoteTally,
    // pub engagement: VoteEngagement,
    pub user_vote: Option<ReviewVote>,
}