use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalOutput {
    pub proposal: Proposal,
    pub role_voting_session_outputs: Vec<ProposalRoleVotingSessionOutput>,
    pub user_votes: Vec<ProposalVote>,
}