use crate::*;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct GroupInput {
    pub name: String,
    pub description: Option<String>,
    pub kind: GroupKindInput,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKindString, ProposalPermission>,
}