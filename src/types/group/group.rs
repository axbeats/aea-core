use crate::*;

pub type GroupId = u64;
pub type VoteOrder = u8;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Group {
    pub id: GroupId,
    pub dao_id: DaoId,
    pub name: String,
    pub kind: GroupKind,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKindString, ProposalPermission>,
}

impl Group {

    pub fn from_input(input: GroupInput, id: GroupId, dao_id: DaoId, staking_id: Option<StakingId>) -> Self {
        Group {
            id,
            dao_id: dao_id,
            name: input.name,
            kind: GroupKind::from_input(input.kind, staking_id),
            vote_weight: input.vote_weight,
            permissions: input.permissions,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupMembers {
    pub members: Vec<AccountId>,
    pub update_method: VoteMethod,
}

impl ChoiceObject for GroupMembers {
    fn set_elected(&mut self, elected: Vec<CandidateId>) {
        // Assume elected candidates are represented as AccountIds in the LocalGroup context.
        self.members = elected
            .into_iter()
            .filter_map(|candidate| match candidate {
                CandidateId::AccountId(account_id) => Some(account_id),
                _ => None, // Ignore other candidate types
            })
            .collect();
    }
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PotentialGroupKind {
    Followers,
    Subscribers,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupInfo {
    pub is_member: bool,
    pub group_size: u64,
}
