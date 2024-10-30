use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupInput {
    pub name: String,
    pub kind: GroupKindInput,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKindString, ProposalPermission>,
}