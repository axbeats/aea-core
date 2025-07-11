use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum ProposalStatus {
    Initializing,
    Voting, // All roles vote simultaneously
    Approved,
    Rejected,
    Spam,
    Expired,
    Failed, // Proposal execution fails
}

impl ProposalStatus {
    pub fn is_voting(&self) -> bool {
        matches!(self, ProposalStatus::Voting)
    }

    pub fn is_final(&self) -> bool {
        matches!(
            self,
            ProposalStatus::Approved
                | ProposalStatus::Rejected
                | ProposalStatus::Spam
                | ProposalStatus::Expired
                | ProposalStatus::Failed
        )
    }
}


#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum RoleVoteStatus {
    VoteOpen,
    VoteClosed(RoleVoteResult),
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub enum RoleVoteResult {
    Approved,
    Rejected,
    Spam,
    Expired,
}