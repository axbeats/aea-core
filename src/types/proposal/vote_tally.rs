use crate::*;

pub type ProposalVoteTallyId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone, PartialEq)]
pub struct ProposalVoteTally {
    pub approve_count: u128,
    pub reject_count: u128,
    pub spam_count: u128,
}

impl Default for ProposalVoteTally {
    fn default() -> Self {
        ProposalVoteTally {
            approve_count: 0,
            reject_count: 0,
            spam_count: 0,
        }
    }
}

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalGroupVoteTally {
    pub group_id: GroupId,
    pub tally: ProposalVoteTally,
}