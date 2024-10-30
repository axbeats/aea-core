use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IndictmentOutput {
    pub indictment: Indictment,
    pub vote_tally: IndictmentVoteTally,
    pub engagement: VoteEngagement,
    pub user_vote: Option<IndictmentVote>,
}