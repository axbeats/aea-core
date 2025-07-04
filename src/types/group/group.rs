use crate::*;

pub type GroupId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub dao_id: DaoId,
    pub video_id: VideoId,
    pub name: String,
    pub description: Option<String>,
    pub kind: GroupKind,
    pub vote_weight: VoteWeightKind,
    pub permissions: HashMap<ProposalKind, ProposalPermission>,
}

impl Group {
    pub fn assert_elected_members(&self) -> &HashSet<AccountId> {
        match &self.kind {
            GroupKind::Elected(group) => &group.members,
            _ => panic!("ERR_NOT_ELECTED"),
        }
    }
}

impl Group {

    pub fn from_input(input: GroupInput, id: GroupId, video_id: VideoId, staking_id: Option<StakingId>) -> Self {
        Group {
            id,
            dao_id: input.dao_id,
            video_id: video_id,
            name: input.name,
            description: input.description,
            kind: GroupKind::from_input(input.kind, staking_id),
            vote_weight: input.vote_weight,
            permissions: input.permissions,
        }
    }
}
