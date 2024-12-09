use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChoiceOutput {
    pub choice: Choice,
    // pub options: Vec<ChoiceOption>,
    pub vote_counts: Vec<ChoiceVoteTally>,
    pub engagement: VoteEngagement,
    pub user_votes: Option<ChoiceVote>,
}