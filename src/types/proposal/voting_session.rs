use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalGroupVotingSession {
    pub group_id: GroupId,
    pub voting_order: u8,
    pub status: GroupVoteStatus,
    pub start_time: Option<u64>,
    pub voting_period: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalGroupVotingSessionOutput {
    pub session: ProposalGroupVotingSession,
    pub vote_tally: ProposalVoteTally,
}