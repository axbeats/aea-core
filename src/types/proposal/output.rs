use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalOutput {
    pub proposal: Proposal,
    pub group_voting_session_outputs: Vec<ProposalGroupVotingSessionOutput>,
    pub engagement: VoteEngagement,
    pub user_votes: Vec<ProposalVote>,
}