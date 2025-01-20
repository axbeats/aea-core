use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ProposalInteraction {
    pub can_vote: bool,
    pub vote_count: Vec<(GroupId, [u128; 3])>,
    pub dao_id: AccountId,
}