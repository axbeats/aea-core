use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct ElectedGroup {
    pub members: HashSet<AccountId>,
    pub vote_method: VoteMethod,
}