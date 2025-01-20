use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalOutput {
    pub proposal: Proposal,
    pub group_voting_session_outputs: Vec<ProposalGroupVotingSessionOutput>,
    pub engagement: VoteEngagement,
    pub user_votes: Vec<ProposalVote>,
}