use crate::*;

pub type GroupId = u64;
pub type VoteOrder = u8;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub dao_id: DaoId,
    pub name: String,
    pub description: Option<String>,
    pub kind: GroupKind,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKindString, ProposalPermission>,
}

impl Group {
    pub fn assert_elected_members(&self) -> &HashSet<AccountId> {
        match &self.kind {
            GroupKind::Elected(members) => members,
            _ => panic!("ERR_NOT_ELECTED"),
        }
    }
}

impl Group {

    pub fn from_input(input: GroupInput, id: GroupId, dao_id: DaoId, staking_id: Option<StakingId>) -> Self {
        Group {
            id,
            dao_id,
            name: input.name,
            description: input.description,
            kind: GroupKind::from_input(input.kind, staking_id),
            vote_weight: input.vote_weight,
            permissions: input.permissions,
        }
    }
}
