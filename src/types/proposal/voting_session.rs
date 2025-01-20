use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalGroupVotingSession {
    pub group_id: GroupId,
    pub voting_order: u8,
    pub status: GroupVoteStatus,
    pub start_time: Option<u64>,
    pub voting_period: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalGroupVotingSessionOutput {
    pub session: ProposalGroupVotingSession,
    pub vote_tally: ProposalVoteTally,
}