use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalRoleVotingSession {
    pub role_id: RoleId,
    pub status: RoleVoteStatus,
    pub start_time: Option<u64>,
    pub voting_period: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalRoleVotingSessionOutput {
    pub session: ProposalRoleVotingSession,
    pub vote_tally: ProposalVoteTally,
}