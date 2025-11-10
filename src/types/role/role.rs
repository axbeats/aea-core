use crate::*;

pub type RoleId = u64;

#[near(serializers = [json, borsh])]
#[derive(Debug, Clone)]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub dao_id: DaoId,
    pub video_id: VideoId,
    pub kind: RoleKind,
    pub permissions: HashMap<ProposalAbility, ProposalPermission>,
}

impl Role {
    pub fn assert_elected_members(&self) -> &HashSet<AccountId> {
        match &self.kind {
            RoleKind::Elected(role) => &role.members,
            _ => panic!("ERR_NOT_ELECTED"),
        }
    }
}

impl Role {

    pub fn from_input(input: RoleInput, id: RoleId, video_id: VideoId, staking_id: Option<StakingId>) -> Self {
        Role {
            id,
            name: input.name,
            dao_id: input.dao_id,
            video_id: video_id,
            kind: RoleKind::from_input(input.kind, staking_id),
            permissions: input.permissions,
        }
    }
}
